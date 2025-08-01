// Code generated by fin-protoc. DO NOT EDIT.
use binary_codec::*;
use bytes::{Buf, BufMut, Bytes, BytesMut};

#[derive(Debug, Clone, PartialEq)]
pub struct CancelReject {
    pub partition_no: i32,
    pub report_index: i64,
    pub appl_id: String,
    pub reporting_pbuid: String,
    pub submitting_pbuid: String,
    pub security_id: String,
    pub security_id_source: String,
    pub owner_type: u16,
    pub clearing_firm: String,
    pub transact_time: i64,
    pub user_info: String,
    pub cl_ord_id: String,
    pub orig_cl_ord_id: String,
    pub account_id: String,
    pub branch_id: String,
    pub ord_status: String,
    pub cxl_rej_reason: u16,
    pub reject_text: String,
    pub order_id: String,
}

impl BinaryCodec for CancelReject {
    fn encode(&self, buf: &mut BytesMut) {
        buf.put_i32_le(self.partition_no);
        buf.put_i64_le(self.report_index);
        put_char_array(buf, &self.appl_id, 3);
        put_char_array(buf, &self.reporting_pbuid, 6);
        put_char_array(buf, &self.submitting_pbuid, 6);
        put_char_array(buf, &self.security_id, 8);
        put_char_array(buf, &self.security_id_source, 4);
        buf.put_u16_le(self.owner_type);
        put_char_array(buf, &self.clearing_firm, 2);
        buf.put_i64_le(self.transact_time);
        put_char_array(buf, &self.user_info, 32);
        put_char_array(buf, &self.cl_ord_id, 10);
        put_char_array(buf, &self.orig_cl_ord_id, 10);
        put_char_array(buf, &self.account_id, 10);
        put_char_array(buf, &self.branch_id, 2);
        put_char_array(buf, &self.ord_status, 1);
        buf.put_u16_le(self.cxl_rej_reason);
        put_char_array(buf, &self.reject_text, 16);
        put_char_array(buf, &self.order_id, 16);
    }

    fn decode(buf: &mut Bytes) -> Option<CancelReject> {
        let partition_no = buf.get_i32_le();
        let report_index = buf.get_i64_le();
        let appl_id = get_char_array(buf, 3)?;
        let reporting_pbuid = get_char_array(buf, 6)?;
        let submitting_pbuid = get_char_array(buf, 6)?;
        let security_id = get_char_array(buf, 8)?;
        let security_id_source = get_char_array(buf, 4)?;
        let owner_type = buf.get_u16_le();
        let clearing_firm = get_char_array(buf, 2)?;
        let transact_time = buf.get_i64_le();
        let user_info = get_char_array(buf, 32)?;
        let cl_ord_id = get_char_array(buf, 10)?;
        let orig_cl_ord_id = get_char_array(buf, 10)?;
        let account_id = get_char_array(buf, 10)?;
        let branch_id = get_char_array(buf, 2)?;
        let ord_status = get_char_array(buf, 1)?;
        let cxl_rej_reason = buf.get_u16_le();
        let reject_text = get_char_array(buf, 16)?;
        let order_id = get_char_array(buf, 16)?;
        Some(Self {
            partition_no,
            report_index,
            appl_id,
            reporting_pbuid,
            submitting_pbuid,
            security_id,
            security_id_source,
            owner_type,
            clearing_firm,
            transact_time,
            user_info,
            cl_ord_id,
            orig_cl_ord_id,
            account_id,
            branch_id,
            ord_status,
            cxl_rej_reason,
            reject_text,
            order_id,
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
            partition_no: -123456,
            report_index: -123456789,
            appl_id: vec!['a'; 3].into_iter().collect::<String>(),
            reporting_pbuid: vec!['a'; 6].into_iter().collect::<String>(),
            submitting_pbuid: vec!['a'; 6].into_iter().collect::<String>(),
            security_id: vec!['a'; 8].into_iter().collect::<String>(),
            security_id_source: vec!['a'; 4].into_iter().collect::<String>(),
            owner_type: 1234,
            clearing_firm: vec!['a'; 2].into_iter().collect::<String>(),
            transact_time: -123456789,
            user_info: vec!['a'; 32].into_iter().collect::<String>(),
            cl_ord_id: vec!['a'; 10].into_iter().collect::<String>(),
            orig_cl_ord_id: vec!['a'; 10].into_iter().collect::<String>(),
            account_id: vec!['a'; 10].into_iter().collect::<String>(),
            branch_id: vec!['a'; 2].into_iter().collect::<String>(),
            ord_status: vec!['a'; 1].into_iter().collect::<String>(),
            cxl_rej_reason: 1234,
            reject_text: vec!['a'; 16].into_iter().collect::<String>(),
            order_id: vec!['a'; 16].into_iter().collect::<String>(),
        };

        let mut buf = BytesMut::new();
        original.encode(&mut buf);
        let mut bytes = buf.freeze();

        let decoded = CancelReject::decode(&mut bytes).unwrap();
        assert_eq!(original, decoded);
    }
}
