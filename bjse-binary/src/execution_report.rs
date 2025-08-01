// Code generated by fin-protoc. DO NOT EDIT.
use binary_codec::*;
use bytes::{Buf, BufMut, Bytes, BytesMut};

use crate::report_extend_010::*;
use crate::report_extend_040::*;
use crate::report_extend_050::*;

#[derive(Debug, Clone, PartialEq)]
pub enum ExecutionReportApplExtendEnum {
    ReportExtend010(ReportExtend010),
    ReportExtend040(ReportExtend040),
    ReportExtend050(ReportExtend050),
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
        put_char_array(buf, &self.order_id, 16);
        put_char_array(buf, &self.cl_ord_id, 10);
        put_char_array(buf, &self.exec_id, 16);
        put_char_array(buf, &self.exec_type, 1);
        put_char_array(buf, &self.ord_status, 1);
        buf.put_i64_le(self.last_px);
        buf.put_i64_le(self.last_qty);
        buf.put_i64_le(self.leaves_qty);
        buf.put_i64_le(self.cum_qty);
        put_char_array(buf, &self.side, 1);
        put_char_array(buf, &self.account_id, 10);
        put_char_array(buf, &self.branch_id, 2);
        match &self.appl_extend {
            ExecutionReportApplExtendEnum::ReportExtend010(msg) => msg.encode(buf),
            ExecutionReportApplExtendEnum::ReportExtend040(msg) => msg.encode(buf),
            ExecutionReportApplExtendEnum::ReportExtend050(msg) => msg.encode(buf),
        }
    }

    fn decode(buf: &mut Bytes) -> Option<ExecutionReport> {
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
        let order_id = get_char_array(buf, 16)?;
        let cl_ord_id = get_char_array(buf, 10)?;
        let exec_id = get_char_array(buf, 16)?;
        let exec_type = get_char_array(buf, 1)?;
        let ord_status = get_char_array(buf, 1)?;
        let last_px = buf.get_i64_le();
        let last_qty = buf.get_i64_le();
        let leaves_qty = buf.get_i64_le();
        let cum_qty = buf.get_i64_le();
        let side = get_char_array(buf, 1)?;
        let account_id = get_char_array(buf, 10)?;
        let branch_id = get_char_array(buf, 2)?;
        let appl_extend = match appl_id.as_str() {
            "010" => ExecutionReportApplExtendEnum::ReportExtend010(ReportExtend010::decode(buf)?),
            "040" => ExecutionReportApplExtendEnum::ReportExtend040(ReportExtend040::decode(buf)?),
            "050" => ExecutionReportApplExtendEnum::ReportExtend050(ReportExtend050::decode(buf)?),
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
            user_info: vec!['a'; 32].into_iter().collect::<String>(),
            order_id: vec!['a'; 16].into_iter().collect::<String>(),
            cl_ord_id: vec!['a'; 10].into_iter().collect::<String>(),
            exec_id: vec!['a'; 16].into_iter().collect::<String>(),
            exec_type: vec!['a'; 1].into_iter().collect::<String>(),
            ord_status: vec!['a'; 1].into_iter().collect::<String>(),
            last_px: -123456789,
            last_qty: -123456789,
            leaves_qty: -123456789,
            cum_qty: -123456789,
            side: vec!['a'; 1].into_iter().collect::<String>(),
            account_id: vec!['a'; 10].into_iter().collect::<String>(),
            branch_id: vec!['a'; 2].into_iter().collect::<String>(),
            appl_id: "010".to_string(),
            appl_extend: ExecutionReportApplExtendEnum::ReportExtend010(ReportExtend010 {
                cash_margin: vec!['a'; 1].into_iter().collect::<String>(),
                settl_type: vec!['a'; 1].into_iter().collect::<String>(),
                settl_period: vec!['a'; 1].into_iter().collect::<String>(),
            }),
        };

        let mut buf = BytesMut::new();
        original.encode(&mut buf);
        let mut bytes = buf.freeze();

        let decoded = ExecutionReport::decode(&mut bytes).unwrap();
        assert_eq!(original, decoded);
    }
}
