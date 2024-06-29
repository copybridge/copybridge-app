use serde::{Deserialize, Serialize};
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

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Config {
    pub server: String,
    pub clipboards: Vec<Clipboard>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Clipboard {
    pub id: u32,
    pub name: String,
    // pub is_encrypted: bool,
    pub password: Option<String>,
}

/// `MyConfig` implements `Default`
impl ::std::default::Default for Config {
    fn default() -> Self { Self { 
        server: "https://copybridge.bitgarden.tech".to_string(),
        clipboards: Vec::new(),
    } }
}

impl Config {
    pub fn read() -> Result<Config, confy::ConfyError> {
        let config: Config = confy::load("copybridge", None)?;
        Ok(config)
    }
    
    pub fn write(self) -> Result<(), confy::ConfyError> {
        confy::store("copybridge", None, self)?;
        Ok(())
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

