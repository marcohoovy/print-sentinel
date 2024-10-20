use std::{collections::VecDeque, thread::spawn};

use crossbeam_channel::{unbounded, Sender};
use xmlrpc::Request;

use crate::{rpc::HOST, THERMAL_RUNAWAY_ERROR_MARGIN, THERMAL_RUNAWAY_INPROGRESS};

pub fn detect_downward_trend(data: VecDeque<[f64; 2]>, margin_of_error: f64) -> bool {
    let mut previous_temp = data[0][0];
    let mut downward_streak = 0;

    for entry in data.iter().skip(1) {
        let current_temp = entry[0];

        if current_temp < previous_temp - margin_of_error {
            downward_streak += 1;
        } else if (current_temp - previous_temp).abs() <= margin_of_error {
            continue;
        } else {
            downward_streak = 0;
        }

        previous_temp = current_temp;

        if downward_streak >= 3 {
            return true;
        }
    }

    false
}

pub fn start_thermal_runaway_detection() -> Sender<[f64; 2]> {
    let (tx, rx) = unbounded();
    spawn(move || {

        let mut buffer = VecDeque::new();

        loop {
            let data: [f64; 2] = rx.recv().unwrap();

            if data[1] == 0.0 { continue; } // Ignore zero temperature (expected cooldowns!)

            buffer.push_back(data);

            unsafe {
                THERMAL_RUNAWAY_INPROGRESS = detect_downward_trend(buffer.clone(), THERMAL_RUNAWAY_ERROR_MARGIN );
            }

            if buffer.len() > 100 { buffer.pop_front(); }

        }

    });

    tx
}

pub fn begin_thermal_runaway_protection() {
    println!("Thermal Runaway Protection Started");
    spawn(move || {
        // Pause the printer
        let _ = Request::new("pauseprint").call_url(HOST);
        // Set the temperature
        let _ = Request::new("settemp").arg("0").call_url(HOST);
        // Set the bed temperature
        let _ = Request::new("setbedtemp").arg("0").call_url(HOST);

        put_printer_into_inspect_mode(false);
    });
}

pub fn put_printer_into_inspect_mode(pause: bool) {
    if pause {
        let _ = Request::new("pauseprint").call_url(HOST);
    }
    let _ = Request::new("send").arg("G91").call_url(HOST);
    let _ = Request::new("send").arg("G1 Z20").call_url(HOST);
    let _ = Request::new("send").arg("G90").call_url(HOST);
}