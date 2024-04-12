use std::io;

use anyerror::AnyError;
use serde::Deserialize;
use serde::Serialize;
use sled::transaction::UnabortableTransactionError;

use crate::MetaBytesError;

/// Storage level error that is raised by meta service.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, thiserror::Error)]
pub enum MetaStorageError {
    /// An error raised when encode/decode data to/from underlying storage.
    #[error(transparent)]
    BytesError(MetaBytesError),

    /// An AnyError built from sled::Error.
    #[error(transparent)]
    SledError(AnyError),

    // TODO(1): remove this error
    /// An internal error that inform txn to retry.
    #[error("Conflict when execute transaction, just retry")]
    TransactionConflict,
}

impl MetaStorageError {
    pub fn name(&self) -> &'static str {
        match self {
            MetaStorageError::BytesError(_) => "BytesError",
            MetaStorageError::SledError(_) => "SledError",
            MetaStorageError::TransactionConflict => "TransactionConflict",
        }
    }
}

impl From<std::string::FromUtf8Error> for MetaStorageError {
    fn from(error: std::string::FromUtf8Error) -> Self {
        MetaStorageError::BytesError(MetaBytesError::new(&error))
    }
}

impl From<serde_json::Error> for MetaStorageError {
    fn from(error: serde_json::Error) -> MetaStorageError {
        MetaStorageError::BytesError(MetaBytesError::new(&error))
    }
}

impl From<MetaBytesError> for MetaStorageError {
    fn from(error: MetaBytesError) -> Self {
        MetaStorageError::BytesError(error)
    }
}

impl From<sled::Error> for MetaStorageError {
    fn from(e: sled::Error) -> MetaStorageError {
        MetaStorageError::SledError(AnyError::new(&e))
    }
}

impl From<UnabortableTransactionError> for MetaStorageError {
    fn from(error: UnabortableTransactionError) -> Self {
        match error {
            UnabortableTransactionError::Storage(e) => {
                MetaStorageError::SledError(AnyError::new(&e))
            }
            UnabortableTransactionError::Conflict => MetaStorageError::TransactionConflict,
        }
    }
}

impl From<MetaStorageError> for io::Error {
    fn from(e: MetaStorageError) -> Self {
        io::Error::new(io::ErrorKind::InvalidData, e)
    }
}
