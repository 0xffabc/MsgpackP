pub trait Serializable {
    fn serialize(&self) -> Vec<u8>;
}

pub trait Deserializable {
    fn deserialize(bytes: &[u8]) -> Self;
}
