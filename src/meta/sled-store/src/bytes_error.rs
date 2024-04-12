use anyerror::AnyError;
use meta_stoerr::MetaBytesError;
use meta_stoerr::MetaStorageError;

/// Errors that occur when encode/decode
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, thiserror::Error)]
#[error("SledBytesError: {source}")]
pub struct SledBytesError {
    #[source]
    pub source: AnyError,
}

impl SledBytesError {
    pub fn new(error: &(impl std::error::Error + 'static)) -> Self {
        Self {
            source: AnyError::new(error),
        }
    }
}

impl From<serde_json::Error> for SledBytesError {
    fn from(e: serde_json::Error) -> Self {
        Self::new(&e)
    }
}

impl From<std::string::FromUtf8Error> for SledBytesError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::new(&e)
    }
}

// TODO: remove this: after refactoring, sled should not use MetaStorageError directly.
impl From<SledBytesError> for MetaStorageError {
    fn from(e: SledBytesError) -> Self {
        MetaStorageError::BytesError(MetaBytesError::new(&e))
    }
}
