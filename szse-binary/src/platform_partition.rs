// Code generated by fin-protoc. DO NOT EDIT.
use binary_codec::*;
use bytes::{Buf, BufMut, Bytes, BytesMut};

#[derive(Debug, Clone, PartialEq)]
pub struct PlatformPartition {
    pub partition_no: i32,
}

impl BinaryCodec for PlatformPartition {
    fn encode(&self, buf: &mut BytesMut) {
        buf.put_i32(self.partition_no);
    }

    fn decode(buf: &mut Bytes) -> Option<PlatformPartition> {
        let partition_no = buf.get_i32();
        Some(Self { partition_no })
    }
}

#[cfg(test)]
mod platform_partition_tests {
    use super::*;
    use bytes::BytesMut;

    #[test]
    fn test_platform_partition_codec() {
        let original = PlatformPartition {
            partition_no: -123456,
        };

        let mut buf = BytesMut::new();
        original.encode(&mut buf);
        let mut bytes = buf.freeze();

        let decoded = PlatformPartition::decode(&mut bytes).unwrap();
        assert_eq!(original, decoded);
    }
}
