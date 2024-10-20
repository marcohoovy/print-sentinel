use std::thread::spawn;

use crossbeam_channel::Sender;
use lazy_static::lazy_static;
use rocket::http::ContentType;
use serde_json::{json, Value};
use tokio::{fs, task::spawn_blocking};
use util::{begin_thermal_runaway_protection, start_thermal_runaway_detection};

lazy_static!(
    pub static ref THERMAL_RUNAWAY_CONTROLLER: Sender<[f64; 2]> = start_thermal_runaway_detection();
);

pub static mut THERMAL_RUNAWAY_ACT: bool = false;
pub static mut THERMAL_RUNAWAY_ERROR_MARGIN: f64 = 1.0;

/// Set to true if the runaway detection is in progress
/// Program Controlled
pub static mut THERMAL_RUNAWAY_INPROGRESS: bool = false;

mod rpc;
mod util;

mod config;

#[cfg(test)]
mod test;

#[macro_use] extern crate rocket;

#[cfg(not(debug_assertions))]
const HOMEPAGE: Option<&str> = Some(include_str!("./index.html"));

#[cfg(debug_assertions)]
const HOMEPAGE: Option<&str> = None;

#[post("/protection/runaway/state/<state>")]
async fn protection_runaway_set(state: bool) { unsafe { THERMAL_RUNAWAY_ACT = state; } }

#[get("/protection/runaway/state")]
async fn protection_runaway_get() -> String { unsafe { THERMAL_RUNAWAY_ACT.to_string() } }

#[post("/protection/runaway/margin/<margin>")]
async fn protection_runaway_set_margin(margin: f64) { unsafe { THERMAL_RUNAWAY_ERROR_MARGIN = margin; } }

#[get("/protection/runaway/margin")]
async fn protection_runaway_get_margin() -> String { unsafe { THERMAL_RUNAWAY_ERROR_MARGIN.to_string() } }

#[get("/protection/runaway/triggered")]
async fn protection_runaway_get_triggered() -> String { unsafe { THERMAL_RUNAWAY_INPROGRESS.to_string() } }


#[get("/")]
async fn index() -> (ContentType, String) {
    if let Some(homepage) = HOMEPAGE {
        (ContentType::HTML, homepage.to_string())
    } else {
        (ContentType::HTML, fs::read_to_string("src/index.html").await.unwrap())
    }
}

#[get("/files")]
async fn files() -> (ContentType, String) {
    // Read the config
    let config = config::Config::load("./config.toml").await;
    
    let mut file_buffer = Vec::new();

    if let Ok(mut files) = tokio::fs::read_dir(config.gcode_store()).await {
        while let Some(file) = files.next_entry().await.unwrap() {

            let filename = file.file_name().to_str().unwrap().to_string();

            file_buffer.push(filename);
            
        }
    }

    let data = json!({
        "data": file_buffer
    });

    (ContentType::JSON, serde_json::to_string(&data).unwrap())
}

#[get("/print/status")]
async fn print_status() -> (ContentType, String) {
    let response = spawn_blocking(|| rpc::get_print_status()).await.unwrap();

    if let Err(e) = THERMAL_RUNAWAY_CONTROLLER.send(response.temps.t) {
        eprintln!("Error sending to thermal runaway controller: {}", e);
    }

    if unsafe { THERMAL_RUNAWAY_ACT } && unsafe { THERMAL_RUNAWAY_INPROGRESS } { begin_thermal_runaway_protection(); }

    (ContentType::JSON, serde_json::to_string(&response).unwrap())
}

#[put("/print/command", data = "<body>")]
fn print_command(body: String) {
    let request: Value = serde_json::from_str(&body).unwrap();

    let command = request.get("command").unwrap().as_str().unwrap();
    let value = if let Some(value) = request.get("value") {
        value.as_str().unwrap()
    } else {
        ""
    };

    let command = command.to_string();
    let value = value.to_string();
    
    println!("Command: {}", command);
    println!("Value: {}", value);

    spawn(move || {
        rpc::send_printer_command(command, value)
    }).join().unwrap();
}

#[launch]
async fn rocket() -> _ {
    rocket::build().mount("/", routes![
        index,
        files,
        print_status,
        print_command,
        protection_runaway_set,
        protection_runaway_get,
        protection_runaway_set_margin,
        protection_runaway_get_margin,
        protection_runaway_get_triggered
    ])
}
