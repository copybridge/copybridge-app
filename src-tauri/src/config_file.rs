use anyhow::{Context, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub server: String,
    pub clipboards: Vec<Clipboard>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Clipboard {
    pub id: u32,
    pub name: String,
    pub is_encrypted: bool,
    pub password: Option<String>,
}

impl Config {
    pub fn new() -> Self {
        Config {
            server: String::new(),
            clipboards: Vec::new(),
        }
    }
    
    pub fn read() -> Result<Config> {
        let config_path = Config::get_path()?;
    
        let config_str = fs::read_to_string(&config_path);
    
        let config: Config = match config_str {
            Ok(config_str) => toml::from_str(&config_str)
                .with_context(|| format!("unable to parse config in {:?}", config_path))?,
            Err(_) => Config::new(),
        };
    
        Ok(config)
    }
    
    pub fn write(self) -> Result<()> {
        let config_path = Config::get_path()?;
    
        let config_str = toml::to_string(&self).context("unable to serialize config")?;
    
        fs::write(config_path.clone(), config_str)
            .with_context(|| format!("unable to write config to {:?}", config_path))?;
    
        Ok(())
    }
    
    fn get_path() -> Result<PathBuf> {
        let proj_dirs = ProjectDirs::from("dev", "abhishekmj303", "copybridge")
            .context("unable to get project directories")?;
    
        let config_dir = proj_dirs.config_dir();
        fs::create_dir_all(config_dir)
            .with_context(|| format!("unable to create config directory: {:?}", config_dir))?;
    
        let config_path = config_dir.join("config.toml");
        // println!("Config path: {:?}", config_path);
    
        Ok(config_path)
    }
}

