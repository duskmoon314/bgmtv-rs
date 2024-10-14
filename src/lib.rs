#![doc = include_str!("../README.md")]

pub mod client;
pub mod types;

pub mod prelude {
    pub use crate::client::Client;

    pub use crate::types::*;
}
