// Code generated by fin-protoc. DO NOT EDIT.
use binary_codec::*;
use bytes::{Buf, BufMut, Bytes, BytesMut};

#[derive(Debug, Clone, PartialEq)]
pub struct Logon {
    pub sender_comp_id: String,
    pub target_comp_id: String,
    pub heart_bt_int: u16,
    pub prtcl_version: String,
    pub trade_date: u32,
    pub q_size: u32,
}

impl BinaryCodec for Logon {
    fn encode(&self, buf: &mut BytesMut) {
        put_char_array(buf, &self.sender_comp_id, 32);
        put_char_array(buf, &self.target_comp_id, 32);
        buf.put_u16(self.heart_bt_int);
        put_char_array(buf, &self.prtcl_version, 8);
        buf.put_u32(self.trade_date);
        buf.put_u32(self.q_size);
    }

    fn decode(buf: &mut Bytes) -> Option<Logon> {
        let sender_comp_id = get_char_array(buf, 32)?;
        let target_comp_id = get_char_array(buf, 32)?;
        let heart_bt_int = buf.get_u16();
        let prtcl_version = get_char_array(buf, 8)?;
        let trade_date = buf.get_u32();
        let q_size = buf.get_u32();
        Some(Self {
            sender_comp_id,
            target_comp_id,
            heart_bt_int,
            prtcl_version,
            trade_date,
            q_size,
        })
    }
}

#[cfg(test)]
mod logon_tests {
    use super::*;
    use bytes::BytesMut;

    #[test]
    fn test_logon_codec() {
        let original = Logon {
            sender_comp_id: vec!['a'; 32].into_iter().collect::<String>(),
            target_comp_id: vec!['a'; 32].into_iter().collect::<String>(),
            heart_bt_int: 1234,
            prtcl_version: vec!['a'; 8].into_iter().collect::<String>(),
            trade_date: 123456,
            q_size: 123456,
        };

        let mut buf = BytesMut::new();
        original.encode(&mut buf);
        let mut bytes = buf.freeze();

        let decoded = Logon::decode(&mut bytes).unwrap();
        assert_eq!(original, decoded);
    }
}
