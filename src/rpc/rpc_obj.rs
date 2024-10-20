use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrintStatus {
    pub filename: Option<String>,
    pub eta: [f64; 3],
    pub temps: TempFields,
    pub progress: Option<f64>,
    pub z: Option<f64>
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TempFields {
    #[serde(rename = "T")]
    pub t: [f64; 2],
    #[serde(rename = "B")]
    pub b: [f64; 2],
}