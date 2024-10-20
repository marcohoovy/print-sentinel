use std::path::Path;

use serde::{Deserialize, Serialize};

use anyhow::Result;


#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub rpc_server: String,
    pub gcode_store: String,   
}

impl Config {
    pub fn gcode_store(&self) -> &Path { Path::new(&self.gcode_store) }

    pub async fn load<P: AsRef<Path>>(path: P) -> Self {
        async fn inner(path: &Path) -> Result<Config> {
            let str = tokio::fs::read_to_string(path).await?;
            let cfg: Config = toml::from_str(&str)?;
            Ok(cfg)
        }
        if let Ok(cfg) = inner(path.as_ref()).await {
            return cfg;
        }
        Self::default()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            rpc_server: "http://localhost:7978".to_string(),
            gcode_store: "./prints/".to_string(),
        }
    }
}