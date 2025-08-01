// Code generated by fin-protoc. DO NOT EDIT.
use binary_codec::*;
use bytes::{Bytes, BytesMut};

#[derive(Debug, Clone, PartialEq)]
pub struct ConfirmExtend043 {}

impl BinaryCodec for ConfirmExtend043 {
    fn encode(&self, _buf: &mut BytesMut) {}

    fn decode(_buf: &mut Bytes) -> Option<ConfirmExtend043> {
        Some(Self {})
    }
}

#[cfg(test)]
mod confirm_extend_043_tests {
    use super::*;
    use bytes::BytesMut;

    #[test]
    fn test_confirm_extend_043_codec() {
        let original = ConfirmExtend043 {};

        let mut buf = BytesMut::new();
        original.encode(&mut buf);
        let mut bytes = buf.freeze();

        let decoded = ConfirmExtend043::decode(&mut bytes).unwrap();
        assert_eq!(original, decoded);
    }
}
