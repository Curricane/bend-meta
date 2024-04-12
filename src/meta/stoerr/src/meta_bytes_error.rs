use anyerror::AnyError;

/// Errors that occur when encode/decode
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, thiserror::Error)]
#[error("MetaBytesError: {source}")]
pub struct MetaBytesError {
    #[source]
    pub source: AnyError,
}

impl MetaBytesError {
    pub fn new(error: &(impl std::error::Error + 'static)) -> Self {
        Self {
            source: AnyError::new(error),
        }
    }
}

impl From<serde_json::Error> for MetaBytesError {
    fn from(e: serde_json::Error) -> Self {
        Self::new(&e)
    }
}

impl From<std::string::FromUtf8Error> for MetaBytesError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::new(&e)
    }
}

impl From<prost::EncodeError> for MetaBytesError {
    fn from(e: prost::EncodeError) -> Self {
        Self::new(&e)
    }
}

impl From<prost::DecodeError> for MetaBytesError {
    fn from(e: prost::DecodeError) -> Self {
        Self::new(&e)
    }
}
