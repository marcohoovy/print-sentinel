use rpc_obj::{PrintStatus, TempFields};
use xmlrpc::Request;

use crate::util::put_printer_into_inspect_mode;

pub mod rpc_obj;

pub const HOST: &str = "http://localhost:7978";

pub fn get_print_status() -> PrintStatus {
    let request = Request::new("status");

    let response = request.call_url(HOST).unwrap();

    #[cfg(debug_assertions)]
    println!("Response: {:?}", response);

    let response = response.as_struct().unwrap();

    let filename = response.get("filename").unwrap().as_str(); // Likely not to fail

    let filename = if let Some(filename) = filename {
        Some(filename.to_string())
    } else {
        Some("?".to_string())
    };

    let progress = response.get("progress").unwrap().as_f64();

    let mut eta_buffer = [-0.0, -0.0, -0.0];
    let eta = response.get("eta").unwrap().as_array();

    if let Some(eta) = eta {
        for (i, num) in eta.into_iter().enumerate() {
            let num = num.as_f64().unwrap();
            eta_buffer[i] = num;
        }
    }

    let temps = response.get("temps").unwrap().as_struct();

    let mut temp_info = TempFields {
        t: [-0.0, -0.0],
        b: [-0.0, -0.0],
    };

    if let Some(temps) = temps {
        let t = temps.get("T").unwrap().as_array().unwrap();
        for (i, num) in t.into_iter().enumerate() {
            let num = num.as_f64().unwrap();
            temp_info.t[i] = num;
        }
        let b = temps.get("B").unwrap().as_array().unwrap();
        for (i, num) in b.into_iter().enumerate() {
            let num = num.as_f64().unwrap();
            temp_info.b[i] = num;
        }
    }

    let z = response.get("z").unwrap().as_f64();

    let print_status = PrintStatus {
        filename: filename,
        eta: eta_buffer,
        temps: temp_info,
        progress: progress,
        z: z
    };

    print_status
}

pub fn send_printer_command(command: String, value: String) {

    let command = command.as_str();
    let value = value.as_str();

    match command {
        "emergency_stop" => {
            let _ = Request::new("pauseprint").call_url(HOST);
            let _ = Request::new("settemp").arg("0").call_url(HOST);
            let _ = Request::new("setbedtemp").arg("0").call_url(HOST);
            let _ = Request::new("send").arg("M112").call_url(HOST);
            let _ = Request::new("send").arg("M999").call_url(HOST);
            println!("Emergency stop");        
        }

        "inspect" => {
            put_printer_into_inspect_mode(true);
            println!("Inspect mode");
        }

        "pause" => {
            let _ = Request::new("pauseprint").call_url(HOST);
            println!("Pause");
        }

        "resume" => {
            let _ = Request::new("resumeprint").call_url(HOST);
            println!("Resume");
        }

        "startprint" => {
            let _ = Request::new("startprint").call_url(HOST);
            println!("Start print");
        }

        "load" => {
            let _ = Request::new("load").arg(value).call_url(HOST);
            println!("Load");
        }

        _ => { println!("Unknown command: {}", command) }
    }


}