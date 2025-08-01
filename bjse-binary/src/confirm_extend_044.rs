// Code generated by fin-protoc. DO NOT EDIT.
use binary_codec::*;
use bytes::{Bytes, BytesMut};

#[derive(Debug, Clone, PartialEq)]
pub struct ConfirmExtend044 {}

impl BinaryCodec for ConfirmExtend044 {
    fn encode(&self, _buf: &mut BytesMut) {}

    fn decode(_buf: &mut Bytes) -> Option<ConfirmExtend044> {
        Some(Self {})
    }
}

#[cfg(test)]
mod confirm_extend_044_tests {
    use super::*;
    use bytes::BytesMut;

    #[test]
    fn test_confirm_extend_044_codec() {
        let original = ConfirmExtend044 {};

        let mut buf = BytesMut::new();
        original.encode(&mut buf);
        let mut bytes = buf.freeze();

        let decoded = ConfirmExtend044::decode(&mut bytes).unwrap();
        assert_eq!(original, decoded);
    }
}
