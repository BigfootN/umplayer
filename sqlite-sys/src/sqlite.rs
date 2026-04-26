use crate::error::{Error, Result};
use crate::sqlite_c_api;
use std::fmt::Write;
use std::path::Path;

/// A connection to a sqlite database.
pub struct Connection {
    handle: *mut sqlite_c_api::sqlite3,
}

impl Connection {
    /// Creates a connection to a sqlite database.\
    /// The database file doesn't need to exist.\
    /// If the database file doesn't exist, it will be created.
    ///
    /// ## Errors
    /// Unable to open or create the database file.
    pub fn new<P>(file_path: &P) -> Result<Self>
    where
        P: AsRef<Path>,
    {
        let file_path = file_path.as_ref().as_os_str().display().to_string();
        let file_path_c_str = std::ffi::CStr::from_bytes_until_nul(file_path.as_bytes())?;
        let mut handle = std::ptr::null_mut::<sqlite_c_api::sqlite3>();
        let err_code =
            unsafe { sqlite_c_api::sqlite3_open(file_path_c_str.as_ptr(), &raw mut handle) }
                .cast_unsigned();
        if err_code == sqlite_c_api::SQLITE_OK {
            Ok(Self { handle })
        } else {
            Err(unsafe { Error::from_sqlite_connection(handle) })
        }
    }

    /// Creates a statement builder linked to this database connection.
    #[must_use]
    pub fn statement(&'_ self) -> StatementBuilder<'_> {
        StatementBuilder {
            connection: self,
            handle: String::new(),
        }
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        unsafe {
            sqlite_c_api::sqlite3_close(self.handle);
        };
    }
}

/// A builder for the sqlite database statement.
pub struct StatementBuilder<'a> {
    connection: &'a Connection,
    handle: String,
}

impl StatementBuilder<'_> {
    pub fn concatenate(mut self, statement_str: &str) -> Self {
        self.handle.push_str(statement_str);
        self
    }

    pub fn add_parameter(mut self, parameter_name: &str) -> Result<Self> {
        write!(&mut self.handle, " :{parameter_name} ")?;
        Ok(self)
    }

    pub fn build(&'_ self) -> Result<Statement<'_>> {
        let mut stmt_handle = std::ptr::null_mut::<sqlite_c_api::sqlite3_stmt>();
        let err_code = self.prepare(&raw mut stmt_handle)?;

        if err_code == sqlite_c_api::SQLITE_OK {
            Ok(Statement {
                handle: stmt_handle,
                _conn_handle: self.connection,
            })
        } else {
            Err(unsafe { Error::from_sqlite_connection(self.connection.handle) })
        }
    }

    fn prepare(&self, stmt_handle: *mut *mut sqlite_c_api::sqlite3_stmt) -> Result<u32> {
        let stmt_str = std::ffi::CStr::from_bytes_until_nul(self.handle.as_bytes())?;
        let mut stmt_tail = std::ptr::null::<std::ffi::c_char>();

        let err_code = unsafe {
            sqlite_c_api::sqlite3_prepare(
                self.connection.handle,
                stmt_str.as_ptr(),
                -1,
                stmt_handle,
                &raw mut stmt_tail,
            )
        }
        .cast_unsigned();

        Ok(err_code)
    }
}

pub enum ParameterValue {
    Double(f64),
    I32(i32),
    I64(i64),
    Text(String),
    Blob(Vec<u8>),
}

impl From<f64> for ParameterValue {
    fn from(value: f64) -> Self {
        Self::Double(value)
    }
}

impl From<i32> for ParameterValue {
    fn from(value: i32) -> Self {
        Self::I32(value)
    }
}

impl From<i64> for ParameterValue {
    fn from(value: i64) -> Self {
        Self::I64(value)
    }
}

impl From<&str> for ParameterValue {
    fn from(value: &str) -> Self {
        Self::Text(value.to_string())
    }
}

impl From<String> for ParameterValue {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}

impl From<Vec<u8>> for ParameterValue {
    fn from(value: Vec<u8>) -> Self {
        Self::Blob(value)
    }
}

pub struct Statement<'a> {
    handle: *mut sqlite_c_api::sqlite3_stmt,
    _conn_handle: &'a Connection,
}

impl Statement<'_> {
    pub fn bind_parameter(&self, parameter_name: &str, value: &ParameterValue) -> Result<()> {
        let parameter_name = format!(":{parameter_name}");
        let parameter_name_c_str = std::ffi::CStr::from_bytes_until_nul(parameter_name.as_bytes())?;
        let parameter_index = unsafe {
            sqlite_c_api::sqlite3_bind_parameter_index(self.handle, parameter_name_c_str.as_ptr())
        };

        let err_code = self.bind_index_parameter(parameter_index, value)?;

        if err_code == sqlite_c_api::SQLITE_OK {
            Ok(())
        } else {
            Err(unsafe { Error::from_sqlite_errcode(err_code) })
        }
    }

    fn bind_index_parameter(&self, parameter_index: i32, value: &ParameterValue) -> Result<u32> {
        let err_code = match value {
            ParameterValue::I32(value) => unsafe {
                sqlite_c_api::sqlite3_bind_int(self.handle, parameter_index, *value)
            },
            ParameterValue::I64(value) => unsafe {
                sqlite_c_api::sqlite3_bind_int64(self.handle, parameter_index, *value)
            },
            ParameterValue::Text(value) => {
                let value_c_str = std::ffi::CStr::from_bytes_until_nul(value.as_bytes())?;
                unsafe {
                    sqlite_c_api::sqlite3_bind_text(
                        self.handle,
                        parameter_index,
                        value_c_str.as_ptr(),
                        -1,
                        None,
                    )
                }
            }
            ParameterValue::Blob(value) => {
                let value = value.clone();
                let (ptr, length, capacity) = value.into_raw_parts();
                let err_code = unsafe {
                    sqlite_c_api::sqlite3_bind_blob(
                        self.handle,
                        parameter_index,
                        ptr.cast::<std::ffi::c_void>(),
                        i32::try_from(length).expect("Blob length must fit in i32"),
                        None,
                    )
                };

                // make sure the memory retrieved from into_raw_parts si freed
                let _ = unsafe { Vec::from_raw_parts(ptr, length, capacity) };

                err_code
            }
            ParameterValue::Double(value) => unsafe {
                sqlite_c_api::sqlite3_bind_double(
                    self.handle,
                    parameter_index,
                    std::ffi::c_double::from(*value),
                )
            },
        }
        .cast_unsigned();

        Ok(err_code)
    }

    pub fn execute(&self) -> Result<()> {
        let err_code = loop {
            let err_code = unsafe { sqlite_c_api::sqlite3_step(self.handle) }.cast_unsigned();
            if err_code != sqlite_c_api::SQLITE_BUSY {
                break err_code;
            }
        };

        if err_code == sqlite_c_api::SQLITE_DONE {
            Ok(())
        } else {
            Err(unsafe { Error::from_sqlite_errcode(err_code) })
        }
    }
}

impl Drop for Statement<'_> {
    fn drop(&mut self) {
        unsafe {
            sqlite_c_api::sqlite3_finalize(self.handle);
        };
    }
}
