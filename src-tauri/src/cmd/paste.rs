use tauri::Url;
use tauri_plugin_http::reqwest;
use crate::cli::PasteArgs;
use crate::config_file::{Clipboard, Config};
use crate::err::Error;

use super::{ApiResponse, Content};


#[tauri::command]
pub async fn set_content(config: Config, args: PasteArgs) -> Result<(), Error> {
    let clipboard: Result<&Clipboard, Error>;
    if args.id.is_some() {
        clipboard = config.get_clipboard(args.id.unwrap())
            .ok_or_else(|| Error::new(
                "Clipboard not found".to_string(),
                format!("ID {}", args.id.unwrap())
            ));
    } else if args.name.is_some() {
        clipboard = config.get_clipboard_by_name(args.name.as_ref().unwrap())
            .ok_or_else(|| Error::new(
                "Clipboard not found".to_string(),
                format!("Name '{}'", args.name.as_ref().unwrap())
            ));
    } else {
        return Err(Error::new(
            "Failed to paste to clipboard".to_string(),
            "ID or Name required".to_string()
        ));
    }

    if args.content.is_none() {
        return Err(Error::new(
            "Failed to paste to clipboard".to_string(),
            "Content required".to_string()
        ));
    }

    let clipboard = match clipboard {
        Ok(clipboard) => clipboard,
        Err(e) => return Err(e),
    };

    let mut url = Url::parse(&config.server).unwrap();
    url.set_path(format!("clipboard/{}", clipboard.id).as_str());

    let body = Content {
        data_type: args.data_type.unwrap_or("text/plain".to_string()),
        data: args.content.unwrap(),
    };

    // GET request with Auth Basic containing password
    let client = reqwest::Client::new();
    let res = client.put(url.clone())
        .basic_auth("user", clipboard.password.clone())
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

    res.json::<ApiResponse>().await
        .map_err(|err| Error::new("Failed to parse server response".to_string(), err.to_string()))?;

    Ok(())
}