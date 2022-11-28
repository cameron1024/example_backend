use color_eyre::Result;

use super::Config;

impl Config {
    pub fn from_env() -> Result<Self> {
        let path = std::env::var("BACKEND_CFG_PATH")?;
        let config = std::fs::read_to_string(path)?;
        let config = serde_json::from_str(&config)?;
        Ok(config)
    }
}
