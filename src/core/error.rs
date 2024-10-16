use thiserror::Error; // for custom error handling
use std::io;
use std::net::SocketAddr;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error), // Convert IO errors
    #[error("Failed to bind to address: {0}")]
    BindError(SocketAddr),
}

