use crate::proto;
use std::str::FromStr;
use uuid::Uuid;

impl TryFrom<proto::Uuid> for Uuid {
    type Error = uuid::Error;

    fn try_from(proto_uuid: proto::Uuid) -> Result<Self, Self::Error> {
        Self::from_str(&proto_uuid.uuid)
    }
}

impl From<Uuid> for proto::Uuid {
    fn from(uuid: Uuid) -> Self {
        Self {
            uuid: uuid.to_string(),
        }
    }
}
