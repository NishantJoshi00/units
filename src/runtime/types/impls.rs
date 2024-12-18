use std::path::PathBuf;

use anyhow::ensure;

impl super::Config {
    pub fn from_path(path: PathBuf) -> anyhow::Result<Self> {
        ensure!(path.exists(), "Config file does not exist");
        ensure!(path.is_file(), "Config file is not a file");

        match path.extension() {
            Some(ext) if ext == "toml" => {
                tracing::info!(?path, "Reading TOML config file");
                let content = std::fs::read_to_string(path)?;
                let config: Self = toml::from_str(&content)?;
                Ok(config)
            }
            Some(ext) if ext == "json" => {
                tracing::info!(?path, "Reading JSON config file");
                let content = std::fs::read_to_string(path)?;
                let config: Self = serde_json::from_str(&content)?;
                Ok(config)
            }
            Some(ext) => anyhow::bail!("Invalid extension: {}", ext.to_string_lossy()),
            None => anyhow::bail!("path must be either a `.json` or `.toml` file"),
        }
    }
}
