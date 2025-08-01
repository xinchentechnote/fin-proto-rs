// Code generated by fin-protoc. DO NOT EDIT.
use binary_codec::*;
use bytes::{Buf, BufMut, Bytes, BytesMut};

use crate::extend_200115::*;
use crate::extend_200215::*;
use crate::extend_200315::*;
use crate::extend_200515::*;
use crate::extend_200615::*;
use crate::extend_200715::*;
use crate::extend_203715::*;
use crate::extend_204115::*;
use crate::extend_204130::*;
use crate::extend_204715::*;
use crate::extend_206315::*;

#[derive(Debug, Clone, PartialEq)]
pub enum ExecutionReportApplExtendEnum {
    Extend200115(Extend200115),
    Extend200215(Extend200215),
    Extend200315(Extend200315),
    Extend200515(Extend200515),
    Extend200615(Extend200615),
    Extend200715(Extend200715),
    Extend206315(Extend206315),
    Extend203715(Extend203715),
    Extend204115(Extend204115),
    Extend204130(Extend204130),
    Extend204715(Extend204715),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExecutionReport {
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
    pub order_id: String,
    pub cl_ord_id: String,
    pub quote_msg_id: String,
    pub exec_id: String,
    pub exec_type: String,
    pub ord_status: String,
    pub last_px: i64,
    pub last_qty: i64,
    pub leaves_qty: i64,
    pub cum_qty: i64,
    pub side: String,
    pub account_id: String,
    pub branch_id: String,
    pub appl_extend: ExecutionReportApplExtendEnum,
}

impl BinaryCodec for ExecutionReport {
    fn encode(&self, buf: &mut BytesMut) {
        buf.put_i32(self.partition_no);
        buf.put_i64(self.report_index);
        put_char_array(buf, &self.appl_id, 3);
        put_char_array(buf, &self.reporting_pbuid, 6);
        put_char_array(buf, &self.submitting_pbuid, 6);
        put_char_array(buf, &self.security_id, 8);
        put_char_array(buf, &self.security_id_source, 4);
        buf.put_u16(self.owner_type);
        put_char_array(buf, &self.clearing_firm, 2);
        buf.put_i64(self.transact_time);
        put_char_array(buf, &self.user_info, 8);
        put_char_array(buf, &self.order_id, 16);
        put_char_array(buf, &self.cl_ord_id, 10);
        put_char_array(buf, &self.quote_msg_id, 10);
        put_char_array(buf, &self.exec_id, 16);
        put_char_array(buf, &self.exec_type, 1);
        put_char_array(buf, &self.ord_status, 1);
        buf.put_i64(self.last_px);
        buf.put_i64(self.last_qty);
        buf.put_i64(self.leaves_qty);
        buf.put_i64(self.cum_qty);
        put_char_array(buf, &self.side, 1);
        put_char_array(buf, &self.account_id, 12);
        put_char_array(buf, &self.branch_id, 4);
        match &self.appl_extend {
            ExecutionReportApplExtendEnum::Extend200115(msg) => msg.encode(buf),
            ExecutionReportApplExtendEnum::Extend200215(msg) => msg.encode(buf),
            ExecutionReportApplExtendEnum::Extend200315(msg) => msg.encode(buf),
            ExecutionReportApplExtendEnum::Extend200515(msg) => msg.encode(buf),
            ExecutionReportApplExtendEnum::Extend200615(msg) => msg.encode(buf),
            ExecutionReportApplExtendEnum::Extend200715(msg) => msg.encode(buf),
            ExecutionReportApplExtendEnum::Extend206315(msg) => msg.encode(buf),
            ExecutionReportApplExtendEnum::Extend203715(msg) => msg.encode(buf),
            ExecutionReportApplExtendEnum::Extend204115(msg) => msg.encode(buf),
            ExecutionReportApplExtendEnum::Extend204130(msg) => msg.encode(buf),
            ExecutionReportApplExtendEnum::Extend204715(msg) => msg.encode(buf),
        }
    }

    fn decode(buf: &mut Bytes) -> Option<ExecutionReport> {
        let partition_no = buf.get_i32();
        let report_index = buf.get_i64();
        let appl_id = get_char_array(buf, 3)?;
        let reporting_pbuid = get_char_array(buf, 6)?;
        let submitting_pbuid = get_char_array(buf, 6)?;
        let security_id = get_char_array(buf, 8)?;
        let security_id_source = get_char_array(buf, 4)?;
        let owner_type = buf.get_u16();
        let clearing_firm = get_char_array(buf, 2)?;
        let transact_time = buf.get_i64();
        let user_info = get_char_array(buf, 8)?;
        let order_id = get_char_array(buf, 16)?;
        let cl_ord_id = get_char_array(buf, 10)?;
        let quote_msg_id = get_char_array(buf, 10)?;
        let exec_id = get_char_array(buf, 16)?;
        let exec_type = get_char_array(buf, 1)?;
        let ord_status = get_char_array(buf, 1)?;
        let last_px = buf.get_i64();
        let last_qty = buf.get_i64();
        let leaves_qty = buf.get_i64();
        let cum_qty = buf.get_i64();
        let side = get_char_array(buf, 1)?;
        let account_id = get_char_array(buf, 12)?;
        let branch_id = get_char_array(buf, 4)?;
        let appl_extend = match appl_id.as_str() {
            "010" => ExecutionReportApplExtendEnum::Extend200115(Extend200115::decode(buf)?),
            "020" => ExecutionReportApplExtendEnum::Extend200215(Extend200215::decode(buf)?),
            "030" => ExecutionReportApplExtendEnum::Extend200315(Extend200315::decode(buf)?),
            "051" => ExecutionReportApplExtendEnum::Extend200515(Extend200515::decode(buf)?),
            "052" => ExecutionReportApplExtendEnum::Extend200515(Extend200515::decode(buf)?),
            "056" => ExecutionReportApplExtendEnum::Extend200515(Extend200515::decode(buf)?),
            "057" => ExecutionReportApplExtendEnum::Extend200515(Extend200515::decode(buf)?),
            "060" => ExecutionReportApplExtendEnum::Extend200615(Extend200615::decode(buf)?),
            "061" => ExecutionReportApplExtendEnum::Extend200615(Extend200615::decode(buf)?),
            "070" => ExecutionReportApplExtendEnum::Extend200715(Extend200715::decode(buf)?),
            "630" => ExecutionReportApplExtendEnum::Extend206315(Extend206315::decode(buf)?),
            "370" => ExecutionReportApplExtendEnum::Extend203715(Extend203715::decode(buf)?),
            "410" => ExecutionReportApplExtendEnum::Extend204115(Extend204115::decode(buf)?),
            "412" => ExecutionReportApplExtendEnum::Extend204115(Extend204115::decode(buf)?),
            "413" => ExecutionReportApplExtendEnum::Extend204115(Extend204115::decode(buf)?),
            "415" => ExecutionReportApplExtendEnum::Extend204115(Extend204115::decode(buf)?),
            "416" => ExecutionReportApplExtendEnum::Extend204115(Extend204115::decode(buf)?),
            "417" => ExecutionReportApplExtendEnum::Extend204130(Extend204130::decode(buf)?),
            "470" => ExecutionReportApplExtendEnum::Extend204715(Extend204715::decode(buf)?),
            _ => return None,
        };
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
            order_id,
            cl_ord_id,
            quote_msg_id,
            exec_id,
            exec_type,
            ord_status,
            last_px,
            last_qty,
            leaves_qty,
            cum_qty,
            side,
            account_id,
            branch_id,
            appl_extend,
        })
    }
}

#[cfg(test)]
mod execution_report_tests {
    use super::*;
    use bytes::BytesMut;

    #[test]
    fn test_execution_report_codec() {
        let original = ExecutionReport {
            partition_no: -123456,
            report_index: -123456789,
            reporting_pbuid: vec!['a'; 6].into_iter().collect::<String>(),
            submitting_pbuid: vec!['a'; 6].into_iter().collect::<String>(),
            security_id: vec!['a'; 8].into_iter().collect::<String>(),
            security_id_source: vec!['a'; 4].into_iter().collect::<String>(),
            owner_type: 1234,
            clearing_firm: vec!['a'; 2].into_iter().collect::<String>(),
            transact_time: -123456789,
            user_info: vec!['a'; 8].into_iter().collect::<String>(),
            order_id: vec!['a'; 16].into_iter().collect::<String>(),
            cl_ord_id: vec!['a'; 10].into_iter().collect::<String>(),
            quote_msg_id: vec!['a'; 10].into_iter().collect::<String>(),
            exec_id: vec!['a'; 16].into_iter().collect::<String>(),
            exec_type: vec!['a'; 1].into_iter().collect::<String>(),
            ord_status: vec!['a'; 1].into_iter().collect::<String>(),
            last_px: -123456789,
            last_qty: -123456789,
            leaves_qty: -123456789,
            cum_qty: -123456789,
            side: vec!['a'; 1].into_iter().collect::<String>(),
            account_id: vec!['a'; 12].into_iter().collect::<String>(),
            branch_id: vec!['a'; 4].into_iter().collect::<String>(),
            appl_id: "010".to_string(),
            appl_extend: ExecutionReportApplExtendEnum::Extend200115(Extend200115 {
                cash_margin: vec!['a'; 1].into_iter().collect::<String>(),
            }),
        };

        let mut buf = BytesMut::new();
        original.encode(&mut buf);
        let mut bytes = buf.freeze();

        let decoded = ExecutionReport::decode(&mut bytes).unwrap();
        assert_eq!(original, decoded);
    }
}
