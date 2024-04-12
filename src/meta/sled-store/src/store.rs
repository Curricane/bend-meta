use crate::SledKeySpace;

/// Defines low level storage API
pub trait Store<KV: SledKeySpace> {
    type Error: std::error::Error;

    fn insert(&self, key: &KV::K, value: &KV::V) -> Result<Option<KV::V>, Self::Error>;

    fn get(&self, key: &KV::K) -> Result<Option<KV::V>, Self::Error>;

    fn remove(&self, key: &KV::K) -> Result<Option<KV::V>, Self::Error>;
}
