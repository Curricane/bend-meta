use meta_types::SeqV;

use crate::SledBytesError;
use crate::SledSerde;

impl SledSerde for String {
    fn de<T: AsRef<[u8]>>(v: T) -> Result<Self, SledBytesError>
    where Self: Sized {
        let s = serde_json::from_slice(v.as_ref())?;
        Ok(s)
    }
}

impl<U> SledSerde for SeqV<U>
where U: serde::Serialize + serde::de::DeserializeOwned
{
    fn de<T: AsRef<[u8]>>(v: T) -> Result<Self, SledBytesError>
    where Self: Sized {
        let s = serde_json::from_slice(v.as_ref())?;
        Ok(s)
    }
}
