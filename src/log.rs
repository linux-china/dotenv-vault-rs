use std::fmt::Display;
use log::{log, Level};

pub fn warn<T>(message: T) where T: Display {
    log!(Level::Warn,
        "[dotenv-vault@{}][Warn] {}",
        env!("CARGO_PKG_VERSION"),
        message
    );
}

pub fn info<T>(message: T) where T: Display {
    log!(Level::Info,
        "[dotenv-vault@{}][Info] {}",
        env!("CARGO_PKG_VERSION"),
        message
    );
}
