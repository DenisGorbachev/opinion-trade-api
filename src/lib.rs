//! This is a module-level comment for a Rust lib

#![deny(clippy::arithmetic_side_effects)]
#![cfg_attr(not(test), deny(unused_crate_dependencies))]

mod rest_client;
pub use rest_client::*;
mod models;
pub use models::*;
mod requests;
pub use requests::*;
