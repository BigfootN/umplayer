//! A sqlite3 library c api wrapper

mod error;
mod sqlite;
mod sqlite_c_api;

pub use error::Error;
pub use error::Result;
pub use sqlite::Connection;
