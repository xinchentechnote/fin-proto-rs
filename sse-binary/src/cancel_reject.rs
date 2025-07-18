// Code generated by fin-protoc. DO NOT EDIT.
use binary_codec::*;
use bytes::{Buf, BufMut, Bytes, BytesMut};

#[derive(Debug, Clone, PartialEq)]
pub struct CancelReject {
    pub pbu: String,
    pub set_id: u32,
    pub report_index: u64,
    pub biz_id: u32,
    pub biz_pbu: String,
    pub cl_ord_id: String,
    pub security_id: String,
    pub orig_cl_ord_id: String,
    pub branch_id: String,
    pub cxl_rej_reason: u32,
    pub trade_date: u32,
    pub transact_time: u64,
    pub user_info: String,
}

impl BinaryCodec for CancelReject {
    fn encode(&self, buf: &mut BytesMut) {
        put_char_array(buf, &self.pbu, 8);
        buf.put_u32(self.set_id);
        buf.put_u64(self.report_index);
        buf.put_u32(self.biz_id);
        put_char_array(buf, &self.biz_pbu, 8);
        put_char_array(buf, &self.cl_ord_id, 10);
        put_char_array(buf, &self.security_id, 12);
        put_char_array(buf, &self.orig_cl_ord_id, 10);
        put_char_array(buf, &self.branch_id, 8);
        buf.put_u32(self.cxl_rej_reason);
        buf.put_u32(self.trade_date);
        buf.put_u64(self.transact_time);
        put_char_array(buf, &self.user_info, 32);
    }

    fn decode(buf: &mut Bytes) -> Option<CancelReject> {
        let pbu = get_char_array(buf, 8)?;
        let set_id = buf.get_u32();
        let report_index = buf.get_u64();
        let biz_id = buf.get_u32();
        let biz_pbu = get_char_array(buf, 8)?;
        let cl_ord_id = get_char_array(buf, 10)?;
        let security_id = get_char_array(buf, 12)?;
        let orig_cl_ord_id = get_char_array(buf, 10)?;
        let branch_id = get_char_array(buf, 8)?;
        let cxl_rej_reason = buf.get_u32();
        let trade_date = buf.get_u32();
        let transact_time = buf.get_u64();
        let user_info = get_char_array(buf, 32)?;
        Some(Self {
            pbu,
            set_id,
            report_index,
            biz_id,
            biz_pbu,
            cl_ord_id,
            security_id,
            orig_cl_ord_id,
            branch_id,
            cxl_rej_reason,
            trade_date,
            transact_time,
            user_info,
        })
    }
}

#[cfg(test)]
mod cancel_reject_tests {
    use super::*;
    use bytes::BytesMut;

    #[test]
    fn test_cancel_reject_codec() {
        let original = CancelReject {
            pbu: vec!['a'; 8].into_iter().collect::<String>(),
            set_id: 123456,
            report_index: 123456789,
            biz_id: 123456,
            biz_pbu: vec!['a'; 8].into_iter().collect::<String>(),
            cl_ord_id: vec!['a'; 10].into_iter().collect::<String>(),
            security_id: vec!['a'; 12].into_iter().collect::<String>(),
            orig_cl_ord_id: vec!['a'; 10].into_iter().collect::<String>(),
            branch_id: vec!['a'; 8].into_iter().collect::<String>(),
            cxl_rej_reason: 123456,
            trade_date: 123456,
            transact_time: 123456789,
            user_info: vec!['a'; 32].into_iter().collect::<String>(),
        };

        let mut buf = BytesMut::new();
        original.encode(&mut buf);
        let mut bytes = buf.freeze();

        let decoded = CancelReject::decode(&mut bytes).unwrap();
        assert_eq!(original, decoded);
    }
}
