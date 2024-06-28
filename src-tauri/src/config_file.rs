use anyhow::{Context, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use crate::err;

#[tauri::command]
pub async fn read_config() -> Result<Config, err::Error> {
    match Config::read() {
        Ok(config) => Ok(config),
        Err(e) => Err(err::Error::new(
            "Failed to read config".to_string(),
            e.to_string()
        )),
    }
}

#[tauri::command]
pub async fn write_config(config: Config) -> Result<(), err::Error> {
    match config.write() {
        Ok(()) => Ok(()),
        Err(e) => Err(err::Error::new(
            "Failed to store config".to_string(),
            e.to_string()
        )),
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub server: String,
    pub clipboards: Vec<Clipboard>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Clipboard {
    pub id: u32,
    pub name: String,
    // pub is_encrypted: bool,
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

    pub fn get_clipboard(&self, id: u32) -> Option<&Clipboard> {
        self.clipboards.iter().find(|c| c.id == id)
    }

    pub fn remove_clipboard(&mut self, id: u32) -> Option<Clipboard> {
        let index = self.clipboards.iter().position(|c| c.id == id);
        match index {
            Some(index) => Some(self.clipboards.remove(index)),
            None => None,
        }
    }

    pub fn get_clipboard_by_name(&self, name: &str) -> Option<&Clipboard> {
        self.clipboards.iter().find(|c| c.name == name)
    }

    pub fn remove_clipboard_by_name(&mut self, name: &str) -> Option<Clipboard> {
        let index = self.clipboards.iter().position(|c| c.name == name);
        match index {
            Some(index) => Some(self.clipboards.remove(index)),
            None => None,
        }
    }
}

