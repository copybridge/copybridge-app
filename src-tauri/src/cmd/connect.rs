use std::collections::HashMap;
use tauri::Url;
use tauri_plugin_http::reqwest;
use crate::config_file::Config;
use crate::err::Error;


#[tauri::command]
pub async fn connect(config: Config, server_url: Url) -> Result<(), Error> {
    let mut config = config;
    println!("Connecting to {} ...", server_url);

    let mut health_url = server_url.clone();
    health_url.set_path("health");
    let res = match reqwest::get(health_url).await {
        Ok(res) => res,
        Err(err) => {
            return Err(Error::new(
                "Failed to connect to server".to_string(),
                err.to_string()
            ));
        }
    };
    
    if res.status() == 200 {
        let data = match res.json::<HashMap<String, String>>().await {
            Ok(data) => data,
            Err(err) => {
                return Err(Error::new(
                    "Failed to parse server response".to_string(),
                    err.to_string()
                ));
            }
        };
        if data["status"] != "up" {
            println!("Server is not up");
            return Ok(());
        }
        println!("Successfully connected to the server");
        config.server = server_url.to_string();
        config.write().unwrap();
    } else {
        return Err(Error::new(
            "invalid server response".to_string(),
            "Failed to connect to server".to_string()
        ));
    }

    Ok(())
}
