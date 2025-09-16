// src/lib.rs
pub mod config;
pub mod grid;
pub mod connections;
pub mod render;
pub mod database;

// Re-export commonly used items for easier access
pub use config::*;
pub use grid::*;
pub use connections::*;
pub use render::*;
pub use database::*;
