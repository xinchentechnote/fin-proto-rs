// Code generated by fin-protoc. DO NOT EDIT.
use binary_codec::*;
use bytes::{Bytes, BytesMut};

#[derive(Debug, Clone, PartialEq)]
pub struct Extend200115 {
    pub cash_margin: char,
}

impl BinaryCodec for Extend200115 {
    fn encode(&self, buf: &mut BytesMut) {
        put_char(buf, self.cash_margin);
    }

    fn decode(buf: &mut Bytes) -> Option<Extend200115> {
        let cash_margin = get_char(buf)?;
        Some(Self { cash_margin })
    }
}

#[cfg(test)]
mod extend_200115_tests {
    use super::*;
    use bytes::BytesMut;

    #[test]
    fn test_extend_200115_codec() {
        let original = Extend200115 { cash_margin: 'a' };

        let mut buf = BytesMut::new();
        original.encode(&mut buf);
        let mut bytes = buf.freeze();

        let decoded = Extend200115::decode(&mut bytes).unwrap();
        assert_eq!(original, decoded);
    }
}
