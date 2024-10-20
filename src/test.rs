use std::collections::VecDeque;

use crate::{config, rpc};



#[test]
fn test_get_print_status() {
    let response = rpc::get_print_status();

    println!("{:?}", response);
}

#[tokio::test]
async fn dir_read_test() {
    let config = config::Config::load("./config.toml").await;

    if let Ok(mut files) = tokio::fs::read_dir(config.gcode_store()).await {
        while let Some(file) = files.next_entry().await.unwrap() {

            println!("{}", file.file_name().to_str().unwrap());
            
        }
    }

}

#[test]
fn test_runaway_detection_warmup() {
    let example_data = vec![
        [0.0, 150.0],
        [10.0, 150.0],
        [20.0, 150.0],
        [30.0, 150.0],
        [40.0, 150.0],
        [50.0, 150.0],
        [60.0, 150.0],
        [70.0, 150.0],
        [80.0, 150.0],
        [90.0, 150.0],
        [100.0, 150.0],
        [110.0, 150.0],
        [120.0, 150.0],
        [130.0, 150.0],
        [140.0, 150.0],
        [150.0, 150.0],
        [151.0, 150.0],
        [149.0, 150.0],
        [148.0, 150.0],
        [150.0, 150.0],
        [151.0, 150.0],
    ];

    let result = crate::util::detect_downward_trend(VecDeque::from(example_data), 1.0);
    assert!(!result);
}

#[test]
#[ignore = "Problem is handled by the thermal runaway detection"]
fn test_runaway_detection_expected_cooldown() {
    let example_data = vec![
        [148.0, 150.0],
        [150.0, 150.0],
        [151.0, 0.0],
        [149.0, 0.0],
        [148.0, 0.0],
        [147.0, 0.0],
        [146.0, 0.0],
        [145.0, 0.0],
        [144.0, 0.0],
        [143.0, 0.0],
        [142.0, 0.0],
        [141.0, 0.0],
        [140.0, 0.0],
        [139.0, 0.0],
        [138.0, 0.0],
        [137.0, 0.0],
        [136.0, 0.0],
        [135.0, 0.0],
        [134.0, 0.0],
        [133.0, 0.0],
        [132.0, 0.0],
        [131.0, 0.0],
        [130.0, 0.0],
        [129.0, 0.0],
    ];

    let result = crate::util::detect_downward_trend(VecDeque::from(example_data), 1.0);
    assert!(!result);
}

#[test]
#[should_panic]
fn test_runaway_detection_unexpected_cooldown() {
    let example_data = vec![
        [150.0, 150.0],
        [151.0, 150.0],
        [149.0, 150.0],
        [148.0, 150.0],
        [150.0, 150.0],
        [149.0, 150.0],
        [148.0, 150.0],
        [147.0, 150.0],
        [146.0, 150.0],
        [145.0, 150.0],
        [144.0, 150.0],
        [143.0, 150.0],
        [142.0, 150.0],
        [141.0, 150.0],
        [140.0, 150.0],
        [139.0, 150.0],
        [138.0, 150.0],
        [137.0, 150.0],
        [136.0, 150.0],
        [135.0, 150.0],
        [134.0, 150.0],
        [133.0, 150.0],
        [132.0, 150.0],
        [131.0, 150.0],
        [130.0, 150.0],
        [129.0, 150.0],
    ];

    let result = crate::util::detect_downward_trend(VecDeque::from(example_data), 1.0);
    assert!(!result);
}

#[test]
#[cfg(feature = "tailscale")]
fn tailscale_ip_test(){ println!("{:?}",crate::util::get_tailscale_ip()); }