use tauri::Url;
use tauri_plugin_http::reqwest;
use crate::cli::{is_cli, CreateArgs};
use crate::config_file::{Clipboard, Config, write_config};
use crate::err::Error;

use super::{ApiResponse, PostBody};

#[tauri::command]
pub async fn create(config: Config, args: CreateArgs) -> Result<Config, Error> {
    let mut config = config;
    let mut args = args;
    println!("Creating a new clipboard: '{}'", &args.name);

    if args.force {
        config.remove_clipboard_by_name(&args.name);
    } else {
        if let Some(clipboard) = config.get_clipboard_by_name(&args.name) {
            return Err(Error::new(
                "Clipboard already exists".to_string(),
                format!("ID {}, Name '{}'", clipboard.id, clipboard.name)
            ));
        }
    }

    if is_cli() && args.password.is_none() {
        println!("Enter a password to encrypt the clipboard. Leave it empty for no password.");
        let password = rpassword::prompt_password("New password : ").unwrap();
        if password != "" {
            let retype_password = rpassword::prompt_password("Retype new password: ").unwrap();
            if password == retype_password {
                args.password = Some(password);
            } else {
                return Err(Error::new(
                    "Failed to create clipboard".to_string(),
                    "passwords doesn't match".to_string()
                ));
            }
        }
    }
    
    if args.password == Some(String::new()) {
        args.password = None;
    }

    let mut url = Url::parse(&config.server).unwrap();
    url.set_path("clipboard");

    let body = PostBody {
        name: args.name,
        data_type: "text/plain".to_string(),
        data: "Hello, from CopyBridge".to_string(),
        is_encrypted: if args.password.is_some() { true } else { false }
    };

    // POST request with Auth Basic containing password
    let client = reqwest::Client::new();
    let res = client.post(url.clone())
        .basic_auth("user", args.password.clone())
        .json(&body)
        .send()
        .await
        .map_err(|err| Error::new("Failed to connect to server".to_string(), err.to_string()))?;

    let status = res.status();
    if status != 200 {
        return Err(Error::new(
            "Response".to_string(),
            format!("{} {}", status.as_u16(), res.text().await.unwrap())
        ));
    }

    let data = res.json::<ApiResponse>().await
        .map_err(|err| Error::new("Failed to parse server response".to_string(), err.to_string()))?;

    config.clipboards.push(Clipboard {
        id: data.id,
        name: data.name.clone(),
        password: args.password,
    });

    write_config(config.clone()).await?;
    println!("Successfully created clipboard ID {}: '{}'", data.id, data.name);

    Ok(config)
}