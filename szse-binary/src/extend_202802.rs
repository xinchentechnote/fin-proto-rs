// Code generated by fin-protoc. DO NOT EDIT.
use binary_codec::*;
use bytes::{Bytes, BytesMut};

#[derive(Debug, Clone, PartialEq)]
pub struct Extend202802 {
    pub lender_pbu: String,
    pub lender_account_id: String,
}

impl BinaryCodec for Extend202802 {
    fn encode(&self, buf: &mut BytesMut) {
        put_char_array(buf, &self.lender_pbu, 6);
        put_char_array(buf, &self.lender_account_id, 12);
    }

    fn decode(buf: &mut Bytes) -> Option<Extend202802> {
        let lender_pbu = get_char_array(buf, 6)?;
        let lender_account_id = get_char_array(buf, 12)?;
        Some(Self {
            lender_pbu,
            lender_account_id,
        })
    }
}

#[cfg(test)]
mod extend_202802_tests {
    use super::*;
    use bytes::BytesMut;

    #[test]
    fn test_extend_202802_codec() {
        let original = Extend202802 {
            lender_pbu: vec!['a'; 6].into_iter().collect::<String>(),
            lender_account_id: vec!['a'; 12].into_iter().collect::<String>(),
        };

        let mut buf = BytesMut::new();
        original.encode(&mut buf);
        let mut bytes = buf.freeze();

        let decoded = Extend202802::decode(&mut bytes).unwrap();
        assert_eq!(original, decoded);
    }
}
