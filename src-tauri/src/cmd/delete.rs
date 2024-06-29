use tauri::Url;
use tauri_plugin_http::reqwest;
use crate::cli::AddDeleteArgs;
use crate::config_file::{Config, write_config};
use crate::err::Error;

#[tauri::command]
pub async fn delete(config: Config, args: AddDeleteArgs) -> Result<Config, Error> {
    let mut config = config;
    let args = args;
    println!("Deleting a clipboard from this device: ID {}", args.id);

    let clipboard = match config.get_clipboard(args.id) {
        Some(clipboard) => clipboard,
        None => {
            return Err(Error::new(
                "Clipboard not found".to_string(),
                format!("ID {}", args.id)
            ));
        }
    };

    let mut url = Url::parse(&config.server).unwrap();
    url.set_path(format!("clipboard/{}", args.id).as_str());

    // DELETE request with Auth Basic containing password
    let client = reqwest::Client::new();
    let res = client.delete(url.clone())
        .basic_auth("user", clipboard.password.clone())
        .send()
        .await
        .map_err(|err| Error::new("Failed to connect to server".to_string(), err.to_string()))?;

    let status = res.status();
    if status != 204 {
        return Err(Error::new(
            "Response".to_string(),
            format!("{} {}", status.as_u16(), res.text().await.unwrap())
        ));
    }

    config.remove_clipboard(args.id);
    write_config(config.clone()).await?;

    Ok(config)
}