use std::mem::size_of_val;
use std::ops::Bound;
use std::ops::RangeBounds;

use byteorder::BigEndian;
use byteorder::ByteOrder;
use serde::de::DeserializeOwned;
use serde::Serialize;
use sled::IVec;

use crate::SledBytesError;

/// Serialize/deserialize(ser/de) to/from sled values.
pub trait SledSerde: Serialize + DeserializeOwned {
    /// (ser)ialize a value to `sled::IVec`.
    fn ser(&self) -> Result<IVec, SledBytesError> {
        let x = serde_json::to_vec(self)?;
        Ok(x.into())
    }

    /// (de)serialize a value from `sled::IVec`.
    fn de<T: AsRef<[u8]>>(v: T) -> Result<Self, SledBytesError>
    where Self: Sized;
}

/// Serialize/deserialize(ser/de) to/from sled values and keeps order after serializing.
///
/// E.g. serde_json does not preserve the order of u64:
/// 9 -> [57], 10 -> [49, 48]
/// While BigEndian encoding preserve the order.
///
/// A type that is used as a sled db key should be serialized with order preserved, such as log index.
pub trait SledOrderedSerde: Serialize + DeserializeOwned {
    /// (ser)ialize a value to `sled::IVec`.
    fn ser(&self) -> Result<IVec, SledBytesError>;

    /// (de)serialize a value from `sled::IVec`.
    fn de<V: AsRef<[u8]>>(v: V) -> Result<Self, SledBytesError>
    where Self: Sized;
}

/// Serialize/deserialize(ser/de) to/from range to sled IVec range.
/// The type must impl SledOrderedSerde so that after serialization the order is preserved.
pub trait SledRangeSerde<SD, V, R>
where
    SD: SledOrderedSerde,
    V: RangeBounds<SD>,
    R: RangeBounds<IVec>,
{
    /// (ser)ialize a range to range of `sled::IVec`.
    fn ser(&self) -> Result<R, SledBytesError>;

    // TODO(xp): do we need this?
    // /// (de)serialize a value from `sled::IVec`.
    // fn de<T: AsRef<[u8]>>(v: T) -> Result<Self, SledBytesError>
    //     where Self: Sized;
}

/// Impl ser/de for range of value that can be ser/de to `sled::IVec`
impl<SD, V> SledRangeSerde<SD, V, (Bound<IVec>, Bound<IVec>)> for V
where
    SD: SledOrderedSerde,
    V: RangeBounds<SD>,
{
    fn ser(&self) -> Result<(Bound<IVec>, Bound<IVec>), SledBytesError> {
        let s = self.start_bound();
        let e = self.end_bound();

        let s = bound_ser(s)?;
        let e = bound_ser(e)?;

        Ok((s, e))
    }
}

fn bound_ser<SD: SledOrderedSerde>(v: Bound<&SD>) -> Result<Bound<sled::IVec>, SledBytesError> {
    let res = match v {
        Bound::Included(v) => Bound::Included(v.ser()?),
        Bound::Excluded(v) => Bound::Excluded(v.ser()?),
        Bound::Unbounded => Bound::Unbounded,
    };
    Ok(res)
}

/// NodeId, LogIndex and Term need to be serialized with order preserved, for listing items.
impl SledOrderedSerde for u64 {
    fn ser(&self) -> Result<IVec, SledBytesError> {
        let size = size_of_val(self);
        let mut buf = vec![0; size];

        BigEndian::write_u64(&mut buf, *self);
        Ok(buf.into())
    }

    /// (de)serialize a value from `sled::IVec`.
    fn de<V: AsRef<[u8]>>(v: V) -> Result<Self, SledBytesError>
    where Self: Sized {
        let res = BigEndian::read_u64(v.as_ref());
        Ok(res)
    }
}

/// For LogId to be able to stored in sled::Tree as a key.
impl SledOrderedSerde for String {
    fn ser(&self) -> Result<IVec, SledBytesError> {
        Ok(IVec::from(self.as_str()))
    }

    fn de<V: AsRef<[u8]>>(v: V) -> Result<Self, SledBytesError>
    where Self: Sized {
        Ok(String::from_utf8(v.as_ref().to_vec())?)
    }
}
