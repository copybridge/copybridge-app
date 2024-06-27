use tauri::Url;
use tauri_plugin_http::reqwest;
use crate::cli::{is_cli, AddDeleteArgs};
use crate::config_file::{Clipboard, Config, write_config};
use crate::err::Error;

use super::ApiResponse;

#[tauri::command]
pub async fn add(config: Config, args: AddDeleteArgs) -> Result<(), Error> {
    let mut config = config;
    let mut args = args;
    println!("Adding a new clipboard to this device: ID {}", args.id);

    if args.force {
        config.remove_clipboard(args.id);
    } else {
        if let Some(clipboard) = config.get_clipboard(args.id) {
            return Err(Error::new(
                "Clipboard already exists".to_string(),
                format!("ID {}, Name '{}'", clipboard.id, clipboard.name)
            ));
        }
    }
    
    let mut url = Url::parse(&config.server).unwrap();
    url.set_path(format!("clipboard/{}", args.id).as_str());

    // GET request with Auth Basic containing password
    let client = reqwest::Client::new();
    let mut res = client.get(url.clone())
        .basic_auth("user", args.password.clone())
        .send()
        .await
        .map_err(|err| Error::new("Failed to connect to server".to_string(), err.to_string()))?;
    
    let mut status = res.status();
    if status != 200 {
        if status == 401 && is_cli() && args.password.is_none() {
            // Input password
            let password = rpassword::prompt_password("Your password: ").unwrap();
            res = client.get(url)
                .basic_auth("user", Some(&password))
                .send()
                .await
                .map_err(|err| Error::new("Failed to connect to server".to_string(), err.to_string()))?;
            status = res.status();
            args.password = Some(password);
        }
        if status != 200 {
            return Err(Error::new(
                "Response".to_string(),
                format!("{} {}", status.as_u16(), res.text().await.unwrap())
            ));
        }
    }

    let data = res.json::<ApiResponse>().await
        .map_err(|err| Error::new("Failed to parse server response".to_string(), err.to_string()))?;

    

    config.clipboards.push(Clipboard {
        id: data.id,
        name: data.name.clone(),
        password: args.password,
    });

    let writing = write_config(config);
    println!("Successfully added clipboard ID {}: {}", data.id, data.name);
    writing.await

}