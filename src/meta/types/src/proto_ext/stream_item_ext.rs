use crate::protobuf as pb;
use crate::protobuf::StreamItem;
use crate::SeqV;

impl StreamItem {
    pub fn new(key: String, value: Option<pb::SeqV>) -> Self {
        StreamItem { key, value }
    }
}

impl From<(String, Option<pb::SeqV>)> for StreamItem {
    fn from(kv: (String, Option<pb::SeqV>)) -> Self {
        StreamItem::new(kv.0, kv.1)
    }
}

impl From<(String, Option<SeqV>)> for StreamItem {
    fn from(kv: (String, Option<SeqV>)) -> Self {
        StreamItem::new(kv.0, kv.1.map(pb::SeqV::from))
    }
}

impl From<(String, SeqV)> for StreamItem {
    fn from(kv: (String, SeqV)) -> Self {
        StreamItem::new(kv.0, Some(pb::SeqV::from(kv.1)))
    }
}
