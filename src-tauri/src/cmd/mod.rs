pub mod connect;
pub use connect::*;

pub mod add;
pub use add::*;

pub mod create;
pub use create::*;

pub mod list;
pub use list::*;

pub mod copy;
pub use copy::*;

pub mod paste;
pub use paste::*;

pub mod remove;
pub use remove::*;

pub mod delete;
pub use delete::*;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct ApiResponse {
    pub id: u32,
    pub name: String,
    #[serde(rename = "type")]
    pub data_type: String,
    pub data: String,
    pub is_encrypted: bool,
}

#[derive(Deserialize, Serialize, Debug)]
struct PostBody {
    pub name: String,
    #[serde(rename = "type")]
    pub data_type: String,
    pub data: String,
    pub is_encrypted: bool,
}

#[derive(Deserialize, Serialize, Debug)]
struct PutBody {
    #[serde(rename = "type")]
    pub data_type: String,
    pub data: String,
}