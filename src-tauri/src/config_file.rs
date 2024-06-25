use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use directories::ProjectDirs;
use anyhow::{Result, Context};

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

pub fn read() -> Result<Config> {
    let config_path = get_config_path()?;
    
    let config_str = fs::read_to_string(&config_path);

    let config: Config = match config_str {
        Ok(config_str) => toml::from_str(&config_str)
            .context("Failed to parse TOML")?,
        Err(_) => Config{
            server: String::new(),
            clipboards: Vec::new(),
        },
    };
    
    Ok(config)
}

pub fn write(config: &Config) -> Result<()> {
    let config_path = get_config_path()?;

    let config_str = toml::to_string(&config)
        .context("Failed to serialize config")?;
    
    fs::write(config_path.clone(), config_str)
        .with_context(|| format!("Failed to write config to {:?}", config_path))?;

    Ok(())
}

fn get_config_path() -> Result<PathBuf> {
    let proj_dirs = ProjectDirs::from("dev", "abhishekmj303", "copybridge") 
        .context("Failed to get project directories")?;
    
    let config_dir = proj_dirs.config_dir();
    fs::create_dir_all(config_dir)
        .with_context(|| format!("Failed to create config directory: {:?}", config_dir))?;

    let config_path = config_dir.join("config.toml");
    println!("Config path: {:?}", config_path);
    
    Ok(config_path)
}