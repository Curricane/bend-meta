use crate::protobuf as pb;
use crate::KVMeta;
use crate::SeqV;

impl From<KVMeta> for pb::KvMeta {
    fn from(m: KVMeta) -> Self {
        Self {
            expire_at: m.get_expire_at_ms().map(|x| x / 1000),
        }
    }
}

impl From<pb::KvMeta> for KVMeta {
    fn from(m: pb::KvMeta) -> Self {
        Self {
            expire_at: m.expire_at,
        }
    }
}

impl pb::KvMeta {
    pub fn new_expire(expire_at: u64) -> Self {
        Self {
            expire_at: Some(expire_at),
        }
    }
}

impl pb::SeqV {
    pub fn new(seq: u64, data: Vec<u8>) -> Self {
        Self {
            seq,
            data,
            meta: None,
        }
    }

    pub fn with_meta(seq: u64, meta: Option<pb::KvMeta>, data: Vec<u8>) -> Self {
        Self { seq, meta, data }
    }
}

impl From<SeqV> for pb::SeqV {
    fn from(sv: SeqV) -> Self {
        Self {
            seq: sv.seq,
            data: sv.data,
            meta: sv.meta.map(pb::KvMeta::from),
        }
    }
}

impl From<pb::SeqV> for SeqV {
    /// Convert from protobuf SeqV to the native SeqV we defined.
    fn from(sv: pb::SeqV) -> Self {
        Self {
            seq: sv.seq,
            data: sv.data,
            meta: sv.meta.map(KVMeta::from),
        }
    }
}
