use crate::sqlite_c_api;
use std::error::Error as StdError;
use std::fmt::Display;

/// Alias [`std::result::Result`] type with [`Error`] as the error type.
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub struct UnknownError {
    code: u32,
    msg: String,
}

impl UnknownError {
    fn new(code: u32, msg: String) -> UnknownError {
        Self { code, msg }
    }

    pub fn code(&self) -> u32 {
        self.code
    }

    pub fn msg(&self) -> &str {
        &self.msg
    }
}

impl UnknownError {
    fn from_sqlite_errcode(err_code: u32) -> UnknownError {
        let err_msg = unsafe { sqlite_c_api::sqlite3_errstr(err_code.cast_signed()) };
        let err_msg_c_str = unsafe { std::ffi::CStr::from_ptr(err_msg) };
        UnknownError::new(err_code, err_msg_c_str.to_str().unwrap().to_string())
    }
}

#[derive(Debug, Clone)]
pub enum Abort {
    Rollback,
    Unknown(UnknownError),
}

impl Abort {
    fn from_sqlite_errcode(err_code: u32) -> Abort {
        match err_code {
            sqlite_c_api::SQLITE_ABORT_ROLLBACK => Abort::Rollback,
            _ => Abort::Unknown(UnknownError::from_sqlite_errcode(err_code)),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Authentication {
    User,
    Unknown(UnknownError),
}

impl Authentication {
    fn from_sqlite_errcode(err_code: u32) -> Authentication {
        match err_code {
            sqlite_c_api::SQLITE_AUTH_USER => Authentication::User,
            _ => Authentication::Unknown(UnknownError::from_sqlite_errcode(err_code)),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Busy {
    Recovery,
    Snapshot,
    Timeout,
    Unknown(UnknownError),
}

impl Busy {
    fn from_sqlite_errcode(err_code: u32) -> Busy {
        match err_code {
            sqlite_c_api::SQLITE_BUSY_RECOVERY => Busy::Recovery,
            sqlite_c_api::SQLITE_BUSY_SNAPSHOT => Busy::Snapshot,
            sqlite_c_api::SQLITE_BUSY_TIMEOUT => Busy::Timeout,
            _ => Busy::Unknown(UnknownError::from_sqlite_errcode(err_code)),
        }
    }
}

#[derive(Debug, Clone)]
pub enum CantOpen {
    PathConversion,
    FullPathConversion,
    IsDirectory,
    IsSymlink,
    Unknown(UnknownError),
}

impl CantOpen {
    fn from_sqlite_errcode(err_code: u32) -> CantOpen {
        match err_code {
            sqlite_c_api::SQLITE_CANTOPEN_CONVPATH => CantOpen::PathConversion,
            sqlite_c_api::SQLITE_CANTOPEN_FULLPATH => CantOpen::FullPathConversion,
            sqlite_c_api::SQLITE_CANTOPEN_ISDIR => CantOpen::IsDirectory,
            sqlite_c_api::SQLITE_CANTOPEN_SYMLINK => CantOpen::IsSymlink,
            _ => CantOpen::Unknown(UnknownError::from_sqlite_errcode(err_code)),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Constraint {
    CheckFailed,
    CommitHookCallbackFailed,
    InvalidDataType,
    ForeignKeyFailed,
    NotNullFailed,
    Pinned,
    PrimaryKeyFailed,
    RowIdNotUnique,
    RaiseFunctionWithinATriggerFired,
    UniqueFailed,
    EntryIsOrWasMissingFromIndex,
    VirtualTableCorrupted,
    Unknown(UnknownError),
}

impl Constraint {
    fn from_sqlite_errcode(err_code: u32) -> Constraint {
        match err_code {
            sqlite_c_api::SQLITE_CONSTRAINT_CHECK => Constraint::CheckFailed,
            sqlite_c_api::SQLITE_CONSTRAINT_COMMITHOOK => Constraint::CommitHookCallbackFailed,
            sqlite_c_api::SQLITE_CONSTRAINT_DATATYPE => Constraint::InvalidDataType,
            sqlite_c_api::SQLITE_CONSTRAINT_FOREIGNKEY => Constraint::ForeignKeyFailed,
            sqlite_c_api::SQLITE_CONSTRAINT_NOTNULL => Constraint::NotNullFailed,
            sqlite_c_api::SQLITE_CONSTRAINT_PINNED => Constraint::Pinned,
            sqlite_c_api::SQLITE_CONSTRAINT_PRIMARYKEY => Constraint::PrimaryKeyFailed,
            sqlite_c_api::SQLITE_CONSTRAINT_ROWID => Constraint::RowIdNotUnique,
            sqlite_c_api::SQLITE_CONSTRAINT_TRIGGER => Constraint::RaiseFunctionWithinATriggerFired,
            sqlite_c_api::SQLITE_CONSTRAINT_UNIQUE => Constraint::UniqueFailed,
            sqlite_c_api::SQLITE_CONSTRAINT_VTAB => Constraint::VirtualTableCorrupted,
            _ => Constraint::Unknown(UnknownError::from_sqlite_errcode(err_code)),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Generic {
    MissingCollatingSequence,
    RetryWithError,
    SnapshotNoLongerAvailable,
    Unknown(UnknownError),
}

impl Generic {
    fn from_sqlite_errcode(err_code: u32) -> Generic {
        match err_code {
            sqlite_c_api::SQLITE_ERROR_MISSING_COLLSEQ => Generic::MissingCollatingSequence,
            sqlite_c_api::SQLITE_ERROR_RETRY => Generic::RetryWithError,
            sqlite_c_api::SQLITE_ERROR_SNAPSHOT => Generic::SnapshotNoLongerAvailable,
            _ => Generic::Unknown(UnknownError::from_sqlite_errcode(err_code)),
        }
    }
}

#[derive(Debug, Clone)]
pub enum InputOutput {
    IoxAccess,
    BeginAtomic,
    CheckReservedBlock,
    Close,
    CommitAtomic,
    PathConversion,
    CorruptedFileSystem,
    ChecksumPageFailed,
    DeletionFailed,
    DeletionObjectDoesNotExist,
    FsyncFailed,
    TemporaryDirectoryFailed,
    LockingFailed,
    MemoryMapOrUnmapFailed,
    OutOfMemory,
    ReadLockFailed,
    DiskReadFailed,
    RollbackAtomic,
    SeekFdFailed,
    ReadBytesSizeFailed,
    FileSizeChangeFailed,
    UnlockFailed,
    WriteFailed,
    SharedCacheLocked,
    Unknown(UnknownError),
}

impl InputOutput {
    fn from_sqlite_errcode(err_code: u32) -> InputOutput {
        match err_code {
            sqlite_c_api::SQLITE_IOERR_ACCESS => InputOutput::IoxAccess,
            sqlite_c_api::SQLITE_IOERR_BEGIN_ATOMIC => InputOutput::BeginAtomic,
            sqlite_c_api::SQLITE_IOERR_CHECKRESERVEDLOCK => InputOutput::CheckReservedBlock,
            sqlite_c_api::SQLITE_IOERR_CLOSE => InputOutput::Close,
            sqlite_c_api::SQLITE_IOERR_COMMIT_ATOMIC => InputOutput::CommitAtomic,
            sqlite_c_api::SQLITE_IOERR_CONVPATH => InputOutput::PathConversion,
            sqlite_c_api::SQLITE_IOERR_FSYNC => InputOutput::FsyncFailed,
            sqlite_c_api::SQLITE_IOERR_GETTEMPPATH => InputOutput::TemporaryDirectoryFailed,
            sqlite_c_api::SQLITE_IOERR_LOCK => InputOutput::LockingFailed,
            sqlite_c_api::SQLITE_IOERR_MMAP => InputOutput::MemoryMapOrUnmapFailed,
            sqlite_c_api::SQLITE_NOMEM => InputOutput::OutOfMemory,
            sqlite_c_api::SQLITE_IOERR_RDLOCK => InputOutput::ReadLockFailed,
            sqlite_c_api::SQLITE_IOERR_READ => InputOutput::DiskReadFailed,
            sqlite_c_api::SQLITE_IOERR_ROLLBACK_ATOMIC => InputOutput::RollbackAtomic,
            sqlite_c_api::SQLITE_IOERR_SEEK => InputOutput::SeekFdFailed,
            sqlite_c_api::SQLITE_IOERR_TRUNCATE => InputOutput::FileSizeChangeFailed,
            sqlite_c_api::SQLITE_IOERR_SHORT_READ => InputOutput::ReadBytesSizeFailed,
            sqlite_c_api::SQLITE_IOERR_UNLOCK => InputOutput::UnlockFailed,
            sqlite_c_api::SQLITE_IOERR_WRITE => InputOutput::WriteFailed,
            sqlite_c_api::SQLITE_IOERR_SHMLOCK => InputOutput::SharedCacheLocked,
            _ => InputOutput::Unknown(UnknownError::from_sqlite_errcode(err_code)),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Locked {
    SharedCache,
    VirtualTable,
    Unknown(UnknownError),
}

impl Locked {
    fn from_sqlite_errcode(err_code: u32) -> Locked {
        match err_code {
            sqlite_c_api::SQLITE_LOCKED_SHAREDCACHE => Locked::SharedCache,
            sqlite_c_api::SQLITE_LOCKED_VTAB => Locked::VirtualTable,
            _ => Locked::Unknown(UnknownError::from_sqlite_errcode(err_code)),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Notice {
    HotJournalRolledBack,
    DatabaseWALModeRecovered,
    Unknown(UnknownError),
}

impl Notice {
    fn from_sqlite_errcode(err_code: u32) -> Notice {
        match err_code {
            sqlite_c_api::SQLITE_NOTICE_RECOVER_ROLLBACK => Notice::HotJournalRolledBack,
            sqlite_c_api::SQLITE_NOTICE_RECOVER_WAL => Notice::DatabaseWALModeRecovered,
            _ => Notice::Unknown(UnknownError::from_sqlite_errcode(err_code)),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Ok {
    LoadPermanently,
    Unknown(UnknownError),
}

impl Ok {
    fn from_sqlite_errcode(err_code: u32) -> Ok {
        match err_code {
            sqlite_c_api::SQLITE_OK_LOAD_PERMANENTLY => Ok::LoadPermanently,
            _ => Ok::Unknown(UnknownError::from_sqlite_errcode(err_code)),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ReadOnly {
    UnableToInitializeOnWALMode,
    UnableToReadLockOnWALMode,
    DatabaseMovedModificationFailed,
    DirectoryPermissionDenied,
    RecoveryDbWritePermissionDenied,
    HotJournalDatabasePermissionDenied,
    Unknown(UnknownError),
}

impl ReadOnly {
    fn from_sqlite_errcode(err_code: u32) -> ReadOnly {
        match err_code {
            sqlite_c_api::SQLITE_READONLY_CANTINIT => ReadOnly::UnableToInitializeOnWALMode,
            sqlite_c_api::SQLITE_READONLY_CANTLOCK => ReadOnly::UnableToReadLockOnWALMode,
            sqlite_c_api::SQLITE_READONLY_DBMOVED => ReadOnly::DatabaseMovedModificationFailed,
            sqlite_c_api::SQLITE_READONLY_DIRECTORY => ReadOnly::DirectoryPermissionDenied,
            sqlite_c_api::SQLITE_READONLY_RECOVERY => ReadOnly::RecoveryDbWritePermissionDenied,
            sqlite_c_api::SQLITE_READONLY_ROLLBACK => ReadOnly::HotJournalDatabasePermissionDenied,
            _ => ReadOnly::Unknown(UnknownError::from_sqlite_errcode(err_code)),
        }
    }
}

#[derive(Clone, Debug)]
pub enum SQLite {
    Abort(Abort),
    Authentication(Authentication),
    Busy(Busy),
    CantOpen(CantOpen),
    Constraint(Constraint),
    Generic(Generic),
    InputOutput(InputOutput),
    Locked(Locked),
    Notice(Notice),
    Ok(Ok),
    ReadOnly(ReadOnly),
    Unknown(UnknownError),
    Empty,
    Format,
    Full,
    Internal,
    Interrupt,
    Mismatch,
    NotFound,
    Permission,
    Protocol,
    Range,
    Row,
    Schema,
    TooBig,
}

impl SQLite {
    fn from_sqlite_errcode(err_code: u32) -> Self {
        let primary_code = err_code & 0xFF;

        match primary_code {
            sqlite_c_api::SQLITE_ABORT => Self::Abort(Abort::from_sqlite_errcode(err_code)),
            sqlite_c_api::SQLITE_AUTH => {
                Self::Authentication(Authentication::from_sqlite_errcode(err_code))
            }
            sqlite_c_api::SQLITE_BUSY => Self::Busy(Busy::from_sqlite_errcode(err_code)),
            sqlite_c_api::SQLITE_CANTOPEN => {
                Self::CantOpen(CantOpen::from_sqlite_errcode(err_code))
            }
            sqlite_c_api::SQLITE_CONSTRAINT => {
                Self::Constraint(Constraint::from_sqlite_errcode(err_code))
            }
            sqlite_c_api::SQLITE_ERROR => Self::Generic(Generic::from_sqlite_errcode(err_code)),
            sqlite_c_api::SQLITE_IOERR => {
                Self::InputOutput(InputOutput::from_sqlite_errcode(err_code))
            }
            sqlite_c_api::SQLITE_LOCKED => Self::Locked(Locked::from_sqlite_errcode(err_code)),
            sqlite_c_api::SQLITE_NOTICE => Self::Notice(Notice::from_sqlite_errcode(err_code)),
            sqlite_c_api::SQLITE_OK => Self::Ok(Ok::from_sqlite_errcode(err_code)),
            sqlite_c_api::SQLITE_READONLY => {
                Self::ReadOnly(ReadOnly::from_sqlite_errcode(err_code))
            }
            sqlite_c_api::SQLITE_EMPTY => Self::Empty,
            sqlite_c_api::SQLITE_FORMAT => Self::Format,
            sqlite_c_api::SQLITE_FULL => Self::Full,
            sqlite_c_api::SQLITE_INTERNAL => Self::Internal,
            sqlite_c_api::SQLITE_INTERRUPT => Self::Interrupt,
            sqlite_c_api::SQLITE_MISMATCH => Self::Mismatch,
            sqlite_c_api::SQLITE_NOTFOUND => Self::NotFound,
            sqlite_c_api::SQLITE_PERM => Self::Permission,
            sqlite_c_api::SQLITE_PROTOCOL => Self::Protocol,
            sqlite_c_api::SQLITE_RANGE => Self::Range,
            sqlite_c_api::SQLITE_ROW => Self::Row,
            sqlite_c_api::SQLITE_SCHEMA => Self::Schema,
            sqlite_c_api::SQLITE_TOOBIG => Self::TooBig,
            _ => Self::Unknown(UnknownError::from_sqlite_errcode(err_code)),
        }
    }
}

#[derive(Clone, Debug)]
pub enum StringEncoding {
    FromBytesUntilNulError(std::ffi::FromBytesUntilNulError),
    ErrorUtf8(std::str::Utf8Error),
}

#[derive(Clone, Debug)]
pub enum ErrorKind {
    SQLite(SQLite),
    StringEncoding(StringEncoding),
    Format(std::fmt::Error),
}

/// The general error for the [`::sqlite_sys`] crate.
#[derive(Debug, Clone)]
pub struct Error {
    kind: ErrorKind,
    msg: String,
}

impl StdError for Error {}

impl Error {
    /// Returns the kind of the error.
    #[must_use]
    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(&self.msg)
    }
}

impl Error {
    /// Creates the general error from the `handle` representing a raw `SQLite` database connection.
    ///
    /// ## Safety
    /// The `handle` must be a valid pointer to an `SQLite` database connection.
    pub unsafe fn from_sqlite_connection(handle: *mut sqlite_c_api::sqlite3) -> Self {
        let err_code = unsafe { sqlite_c_api::sqlite3_extended_errcode(handle) }.cast_unsigned();

        unsafe { Self::from_sqlite_errcode(err_code) }
    }

    /// Creates the general error from the `err_code` representing an ` SQLite ` code error.
    ///
    /// ## Panics
    /// The message representing the error contains invalid UTF-8 characters.
    ///
    /// ## Safety
    #[must_use]
    pub unsafe fn from_sqlite_errcode(err_code: u32) -> Self {
        let msg_c_str = unsafe { sqlite_c_api::sqlite3_errstr(err_code.cast_signed()) };
        let msg = unsafe { std::ffi::CStr::from_ptr(msg_c_str) };

        Self {
            kind: ErrorKind::SQLite(SQLite::from_sqlite_errcode(err_code)),
            msg: msg.to_str().unwrap().to_string(),
        }
    }
}

impl From<std::ffi::FromBytesUntilNulError> for Error {
    fn from(error: std::ffi::FromBytesUntilNulError) -> Self {
        Self {
            kind: ErrorKind::StringEncoding(StringEncoding::FromBytesUntilNulError(error.clone())),
            msg: error.to_string(),
        }
    }
}

impl From<std::fmt::Error> for Error {
    fn from(value: std::fmt::Error) -> Self {
        Self {
            kind: ErrorKind::Format(value),
            msg: value.to_string(),
        }
    }
}
