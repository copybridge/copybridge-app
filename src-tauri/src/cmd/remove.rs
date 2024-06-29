use crate::cli::RemoveArgs;
use crate::config_file::{Config, write_config};
use crate::err::Error;

#[tauri::command]
pub async fn remove(config: Config, args: RemoveArgs) -> Result<Config, Error> {
    let mut config = config;
    println!("Removing a clipboard: ID {}, Name '{}'", args.id.unwrap_or(0), args.name.clone().unwrap_or("".to_string()));

    if args.id.is_none() && args.name.is_none() {
        return Err(Error::new(
            "No ID or Name provided".to_string(),
            "Please provide an ID or Name to remove a clipboard".to_string()
        ));
    }

    if let Some(id) = args.id {
        config.remove_clipboard(id);
    } else if let Some(name) = args.name {
        config.remove_clipboard_by_name(&name);
    }

    write_config(config.clone()).await?;

    Ok(config)
}