// Code generated by fin-protoc. DO NOT EDIT.
use binary_codec::*;
use bytes::{Bytes, BytesMut};

#[derive(Debug, Clone, PartialEq)]
pub struct EmptyPacket {}

impl BinaryCodec for EmptyPacket {
    fn encode(&self, _buf: &mut BytesMut) {}

    fn decode(_buf: &mut Bytes) -> Option<EmptyPacket> {
        Some(Self {})
    }
}

#[cfg(test)]
mod empty_packet_tests {
    use super::*;
    use bytes::BytesMut;

    #[test]
    fn test_empty_packet_codec() {
        let original = EmptyPacket {};

        let mut buf = BytesMut::new();
        original.encode(&mut buf);
        let mut bytes = buf.freeze();

        let decoded = EmptyPacket::decode(&mut bytes).unwrap();
        assert_eq!(original, decoded);
    }
}
