use std::{path::PathBuf, process};
use clap::{ArgGroup, Args, Parser, Subcommand};
use tauri_plugin_clipboard_manager::ClipboardExt;
use tauri::Url;
use tokio::runtime::Runtime;
use serde::{Deserialize, Serialize};
use crate::config_file::Config;
use crate::cmd;

use std::sync::atomic::{AtomicBool, Ordering};

pub static IS_CLI: AtomicBool = AtomicBool::new(false);

fn set_is_cli(value: bool) {
    IS_CLI.store(value, Ordering::SeqCst);
}

pub fn is_cli() -> bool {
    IS_CLI.load(Ordering::SeqCst)
}

/// Simple program to greet a person
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Connect the app to a server
    Connect { server_url: Url },

    /// Create a new clipboard and add to this device
    Create(CreateArgs),

    /// Add a clipboard to this device
    Add(AddDeleteArgs),

    /// List all clipboards present in this device
    List,

    /// Copy the content from remote clipboard
    Copy(CopyArgs),

    /// Paste the content into remote clipboard
    Paste(PasteArgs),

    /// Remove a clipboard from this device
    Remove(RemoveArgs),

    /// Delete a clipboard in server and from all the devices
    Delete(AddDeleteArgs),
}

#[derive(Args, Clone, Debug, Deserialize, Serialize)]
pub struct CreateArgs {
    /// Name of the clipboard to create
    pub name: String,

    /// Password for the clipboard if you want to encrypt it
    #[arg(short, long)]
    pub password: Option<String>,

    /// Force the operation
    #[arg(short, long)]
    pub force: bool,
}

#[derive(Args, Clone, Debug, Deserialize, Serialize)]
pub struct AddDeleteArgs {
    /// Clipboard ID from other device
    pub id: u32,

    /// Password for the clipboard if it is encrypted
    #[arg(short, long)]
    pub password: Option<String>,

    /// Force the operation
    #[arg(short, long)]
    pub force: bool,
}

#[derive(Args, Clone, Debug, Deserialize, Serialize)]
#[command(group(
    ArgGroup::new("copy_from")
        .required(true)
        .args(&["id", "name"])
))]
pub struct CopyArgs {
    /// The clipboard ID to copy
    #[arg(short, long)]
    pub id: Option<u32>,

    /// The name of the clipboard to copy
    #[arg(short, long)]
    pub name: Option<String>,

    // /// Password for the clipboard if it is encrypted
    // #[arg(short, long)]
    // pub password: Option<String>,

    /// Print the copied content into console
    #[arg(long)]
    pub echo: bool,
}

#[derive(Args, Clone, Debug, Deserialize, Serialize)]
#[command(group(
    ArgGroup::new("paste_to")
        .required(true)
        .args(&["id", "name"])
))]
#[command(group(
    ArgGroup::new("content_from")
        .required(true)
        .args(&["content", "file"])
))]
pub struct PasteArgs {
    /// The clipboard ID to copy
    #[arg(short, long)]
    pub id: Option<u32>,

    /// The name of the clipboard to copy
    #[arg(short, long)]
    pub name: Option<String>,

    // /// Password for the clipboard if it is encrypted
    // #[arg(short, long)]
    // pub password: Option<String>,

    /// Type of the content to be pasted
    #[arg(short, long)]
    pub data_type: Option<String>,

    /// Content to be pasted
    pub content: Option<String>,

    /// Content of the file to be pasted
    #[arg(short, long)]
    pub file: Option<PathBuf>,
}

#[derive(Args, Clone, Debug, Deserialize, Serialize)]
#[command(group(
    ArgGroup::new("remove_by")
        .required(true)
        .args(&["id", "name"])
))]
pub struct RemoveArgs {
    /// The clipboard ID to remove
    pub id: Option<u32>,

    /// The name of the clipboard to remove
    #[arg(short, long)]
    pub name: Option<String>,
}

pub fn handle_cli() {
    let cli = Cli::parse();

    match &cli.command {
        Some(command) => {
            set_is_cli(true);
            let config = Config::read().unwrap();
            let mut exit_code = 0;
            match command {
                Commands::Connect { server_url } => {
                    exit_code = cli_connect(config, server_url);
                }
                Commands::Create(args) => {
                    exit_code = cli_create(config, args);
                }
                Commands::Add(args) => {
                    exit_code = cli_add(config, args);
                }
                Commands::List => {
                    cmd::list();
                }
                Commands::Copy(args) => {
                    exit_code = cli_copy(config, args);
                }
                Commands::Paste(args) => {
                    exit_code = cli_paste(config, args);
                }
                Commands::Remove(args) => {
                    exit_code = cli_remove(config, args);
                }
                Commands::Delete(args) => {
                    exit_code = cli_delete(config, args);
                } // _ => {
                  //     println!("Not implemented yet!!");
                  // }
            }
            process::exit(exit_code);
        }
        None => {}
    }
}

fn cli_connect(config: Config, server_url: &Url) -> i32 {
    match Runtime::new().unwrap().block_on(cmd::connect(config, server_url.clone())) {
        Ok(()) => 0,
        Err(err) => {
            println!("{}: {}", err.title, err.message);
            err.code
        }
    }
}

fn cli_create(config: Config, args: &CreateArgs) -> i32 {
    match Runtime::new().unwrap().block_on(cmd::create(config, args.clone())) {
        Ok(_) => 0,
        Err(err) => {
            println!("{}: {}", err.title, err.message);
            err.code
        }
    }
}

fn cli_add(config: Config, args: &AddDeleteArgs) -> i32 {
    match Runtime::new().unwrap().block_on(cmd::add(config, args.clone())) {
        Ok(_) => 0,
        Err(err) => {
            println!("{}: {}", err.title, err.message);
            err.code
        }
    }
}

fn cli_copy(config: Config, args: &CopyArgs) -> i32 {
    // let mut ctx = ClipboardManager::new().unwrap();

    let content = match Runtime::new().unwrap().block_on(cmd::get_content(config, args.clone())) {
        Ok(content) => content,
        Err(err) => {
            println!("{}: {}", err.title, err.message);
            return err.code
        }
    };

    let app = tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .build(tauri::generate_context!())
        .expect("msg");

    // let clipboard_content = tauri_plugin_clipboard_manager::ClipboardExt:: {
    //     label: Some("Label".to_string()),
    //     text: "Tauri is awesome!".to_string(),
    // };

    app.clipboard().write_text(content.data).unwrap();

    // tauri_plugin_clipboard_manager::init()

    // ctx.set().wait().clipboard(LinuxClipboardKind::Primary).text(content.data.clone()).unwrap();

    // match Runtime::new().unwrap().block_on(cmd::copy(config, args.clone())) {
    //     Ok(content) => content,
    //     Err(err) => {
    //         println!("{}: {}", err.title, err.message);
    //         return err.code
    //     }
    // };


    return 0
}

fn cli_paste(config: Config, args: &PasteArgs) -> i32 {
    match Runtime::new().unwrap().block_on(cmd::set_content(config, args.clone())) {
        Ok(()) => 0,
        Err(err) => {
            println!("{}: {}", err.title, err.message);
            err.code
        }
    }
}

fn cli_remove(config: Config, args: &RemoveArgs) -> i32 {
    match Runtime::new().unwrap().block_on(cmd::remove(config, args.clone())) {
        Ok(_) => 0,
        Err(err) => {
            println!("{}: {}", err.title, err.message);
            err.code
        }
    }
}

fn cli_delete(config: Config, args: &AddDeleteArgs) -> i32 {
    match Runtime::new().unwrap().block_on(cmd::delete(config, args.clone())) {
        Ok(_) => 0,
        Err(err) => {
            println!("{}: {}", err.title, err.message);
            err.code
        }
    }
}