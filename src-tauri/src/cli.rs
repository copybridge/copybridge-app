use std::process;

use clap::{ArgGroup, Args, Parser, Subcommand};

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
    Connect { address: String },

    /// Create a new clipboard and add to this device
    Create { name: String },

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

#[derive(Args, Debug)]
struct AddDeleteArgs {
    /// Clipboard ID from other device
    id: u32,

    /// Password for the clipboard if it is encrypted
    #[arg(short, long)]
    password: Option<String>,
}

#[derive(Args, Debug)]
#[command(group(
    ArgGroup::new("copy_from")
        .required(true)
        .args(&["id", "name"])
))]
struct CopyArgs {
    /// The clipboard ID to copy
    #[arg(short, long)]
    id: Option<u32>,

    /// The name of the clipboard to copy
    #[arg(short, long)]
    name: Option<String>,

    /// Password for the clipboard if it is encrypted
    #[arg(short, long)]
    password: Option<String>,

    /// Print the copied content into console
    #[arg(long)]
    echo: bool,
}

#[derive(Args, Debug)]
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
struct PasteArgs {
    /// The clipboard ID to copy
    #[arg(short, long)]
    id: Option<u32>,

    /// The name of the clipboard to copy
    #[arg(short, long)]
    name: Option<String>,

    /// Password for the clipboard if it is encrypted
    #[arg(short, long)]
    password: Option<String>,

    /// Content to be pasted
    content: Option<String>,

    /// Content of the file to be pasted
    #[arg(short, long)]
    file: Option<String>,
}

#[derive(Parser, Debug)]
#[command(group(
    ArgGroup::new("remove_by")
        .required(true)
        .args(&["id", "name"])
))]
struct RemoveArgs {
    /// The clipboard ID to remove
    id: Option<u32>,

    /// The name of the clipboard to remove
    #[arg(short, long)]
    name: Option<String>,
}

pub fn handle_cli() {
    let cli = Cli::parse();

    match &cli.command {
        Some(command) => {
            match command {
                Commands::Connect { address } => {
                    connect_command(address);
                }
                Commands::Add(args) => {
                    add_command(args);
                }
                Commands::Create { name } => {
                    create_command(name);
                }
                Commands::List => {
                    list_command();
                }
                Commands::Copy(args) => {
                    copy_command(args);
                }
                Commands::Paste(args) => {
                    paste_command(args);
                }
                Commands::Remove(args) => {
                    remove_command(args);
                }
                Commands::Delete(args) => {
                    delete_command(args);
                }
                // _ => {
                //     println!("Not implemented yet!!");
                // }
            }
            process::exit(0);
        }
        None => {}
    }
}

fn connect_command(address: &String) {
    println!("Command not implemented yet");
    println!("{:?}", address);
}

fn add_command(args: &AddDeleteArgs) {
    println!("Command not implemented yet");
    println!("{:?}", args);
}

fn create_command(name: &String) {
    println!("Command not implemented yet");
    println!("{:?}", name);
}

fn list_command() {
    println!("Command not implemented yet");
    println!("list");
}

fn copy_command(args: &CopyArgs) {
    println!("Command not implemented yet");
    println!("{:?}", args);
}

fn paste_command(args: &PasteArgs) {
    println!("Command not implemented yet");
    println!("{:?}", args);
}

fn remove_command(args: &RemoveArgs) {
    println!("Command not implemented yet");
    println!("{:?}", args);
}

fn delete_command(args: &AddDeleteArgs) {
    println!("Command not implemented yet");
    println!("{:?}", args);
}
