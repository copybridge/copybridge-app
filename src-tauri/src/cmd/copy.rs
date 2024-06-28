use tauri::Url;
use tauri_plugin_http::reqwest;
// use arboard::Clipboard as ClipboardManager;
// use tokio::time::{sleep, Duration};
use crate::cli::CopyArgs;
use crate::config_file::{Clipboard, Config};
use crate::err::Error;

use super::{ApiResponse, Content};

// pub async fn copy(config: Config, args: CopyArgs) -> Result<(), Error> {
//     let content = get_content(config, args.clone()).await?;
//     let mut clipboard_manager = ClipboardManager::new().unwrap();
//     if content.data_type == "text/plain" {
//         if args.echo {
//             println!("{}", content.data.clone());
//         } else {
//             println!("Copying text to clipboard")
//         }
//         println!("{}", clipboard_manager.get_text().unwrap());
//         match clipboard_manager.set_text(content.data) {
//             Ok(_) => {},
//             Err(e) => return Err(Error::new("Failed to copy text to clipboard".to_string(), e.to_string())),
//         }
//         sleep(Duration::from_millis(1000)).await;
//         println!("{}", clipboard_manager.get_text().unwrap());
//     } else if content.data_type == "image/png" {
//         println!("Unsupported data type: 'image/png'")
//         // println!("Copying image to clipboard");
//         // clipboard_manager.set_image(data.data);
//     } else {
//         println!("Unsupported data type: '{}'", content.data_type);
//     }

//     Ok(())
// }


#[tauri::command]
pub async fn get_content(config: Config, args: CopyArgs) -> Result<Content, Error> {
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
            "Failed to copy from clipboard".to_string(),
            "ID or Name required".to_string()
        ));
    }
    
    let clipboard = match clipboard {
        Ok(clipboard) => clipboard,
        Err(e) => return Err(e),
    };

    let mut url = Url::parse(&config.server).unwrap();
    url.set_path(format!("clipboard/{}", clipboard.id).as_str());

    // GET request with Auth Basic containing password
    let client = reqwest::Client::new();
    let res = client.get(url.clone())
        .basic_auth("user", clipboard.password.clone())
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

    Ok(Content {
        data_type: data.data_type,
        data: data.data,
    })
}