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

impl PartialEq<proto::Uuid> for Uuid {
    fn eq(&self, other: &proto::Uuid) -> bool {
        self.to_string() == other.uuid
    }
}

#[cfg(test)]
mod tests {
    use crate::proto;
    use uuid::Uuid;

    /// Ensure that [`uuid::Uuid`] and [`proto::Uuid`] can be converted back
    /// and forth losslessly if the [`proto::Uuid`] instance maintains its
    /// invariants.
    #[test]
    fn conversion_roundtrip() {
        let uuid = Uuid::new_v4();
        let proto_uuid: proto::Uuid = uuid.into();
        let parsed_uuid: Result<Uuid, _> = proto_uuid.try_into();
        assert_eq!(parsed_uuid, Ok(uuid));
    }

    /// Ensure a malfromed [`proto::Uuid`] can't become a [`uuid::Uuid`].
    #[test]
    fn malformed_from_proto() {
        let uuid: &str = "this-is-4sure-wrong";
        let proto_uuid = proto::Uuid { uuid: uuid.into() };
        assert!(Uuid::try_from(proto_uuid).is_err());
    }
}
