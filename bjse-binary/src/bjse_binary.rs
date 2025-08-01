// Code generated by fin-protoc. DO NOT EDIT.
use binary_codec::*;
use bytes::{Buf, BufMut, Bytes, BytesMut};

use crate::cancel_reject::*;
use crate::execution_confirm::*;
use crate::execution_report::*;
use crate::heartbeat::*;
use crate::logon::*;
use crate::logout::*;
use crate::new_order::*;
use crate::order_cancel_request::*;
use crate::platform_state_info::*;
use crate::report_finished::*;
use crate::report_synchronization::*;

#[derive(Debug, Clone, PartialEq)]
pub enum BjseBinaryBodyEnum {
    Logon(Logon),
    Logout(Logout),
    Heartbeat(Heartbeat),
    NewOrder(NewOrder),
    OrderCancelRequest(OrderCancelRequest),
    CancelReject(CancelReject),
    ExecutionConfirm(ExecutionConfirm),
    ExecutionReport(ExecutionReport),
    ReportSynchronization(ReportSynchronization),
    PlatformStateInfo(PlatformStateInfo),
    ReportFinished(ReportFinished),
}

#[derive(Debug, Clone, PartialEq)]
pub struct BjseBinary {
    pub msg_type: u32,
    pub body_length: u32,
    pub body: BjseBinaryBodyEnum,
    pub checksum: u32,
}

impl BinaryCodec for BjseBinary {
    fn encode(&self, buf: &mut BytesMut) {
        buf.put_u32_le(self.msg_type);
        buf.put_u32_le(self.body_length);
        match &self.body {
            BjseBinaryBodyEnum::Logon(msg) => msg.encode(buf),
            BjseBinaryBodyEnum::Logout(msg) => msg.encode(buf),
            BjseBinaryBodyEnum::Heartbeat(msg) => msg.encode(buf),
            BjseBinaryBodyEnum::NewOrder(msg) => msg.encode(buf),
            BjseBinaryBodyEnum::OrderCancelRequest(msg) => msg.encode(buf),
            BjseBinaryBodyEnum::CancelReject(msg) => msg.encode(buf),
            BjseBinaryBodyEnum::ExecutionConfirm(msg) => msg.encode(buf),
            BjseBinaryBodyEnum::ExecutionReport(msg) => msg.encode(buf),
            BjseBinaryBodyEnum::ReportSynchronization(msg) => msg.encode(buf),
            BjseBinaryBodyEnum::PlatformStateInfo(msg) => msg.encode(buf),
            BjseBinaryBodyEnum::ReportFinished(msg) => msg.encode(buf),
        }
        buf.put_u32_le(self.checksum);
    }

    fn decode(buf: &mut Bytes) -> Option<BjseBinary> {
        let msg_type = buf.get_u32_le();
        let body_length = buf.get_u32_le();
        let body = match msg_type {
            1 => BjseBinaryBodyEnum::Logon(Logon::decode(buf)?),
            2 => BjseBinaryBodyEnum::Logout(Logout::decode(buf)?),
            3 => BjseBinaryBodyEnum::Heartbeat(Heartbeat::decode(buf)?),
            101000 => BjseBinaryBodyEnum::NewOrder(NewOrder::decode(buf)?),
            102000 => BjseBinaryBodyEnum::OrderCancelRequest(OrderCancelRequest::decode(buf)?),
            201000 => BjseBinaryBodyEnum::CancelReject(CancelReject::decode(buf)?),
            202010 => BjseBinaryBodyEnum::ExecutionConfirm(ExecutionConfirm::decode(buf)?),
            203010 => BjseBinaryBodyEnum::ExecutionReport(ExecutionReport::decode(buf)?),
            5 => BjseBinaryBodyEnum::ReportSynchronization(ReportSynchronization::decode(buf)?),
            6 => BjseBinaryBodyEnum::PlatformStateInfo(PlatformStateInfo::decode(buf)?),
            7 => BjseBinaryBodyEnum::ReportFinished(ReportFinished::decode(buf)?),
            _ => return None,
        };
        let checksum = buf.get_u32_le();
        Some(Self {
            msg_type,
            body_length,
            body,
            checksum,
        })
    }
}

#[cfg(test)]
mod bjse_binary_tests {
    use super::*;
    use bytes::BytesMut;

    #[test]
    fn test_bjse_binary_codec() {
        let original = BjseBinary {
            body_length: 123456,
            msg_type: 1,
            body: BjseBinaryBodyEnum::Logon(Logon {
                sender_comp_id: vec!['a'; 20].into_iter().collect::<String>(),
                target_comp_id: vec!['a'; 20].into_iter().collect::<String>(),
                heart_bt_int: -123456,
                password: vec!['a'; 16].into_iter().collect::<String>(),
                default_appl_ver_id: vec!['a'; 32].into_iter().collect::<String>(),
            }),
            checksum: 123456,
        };

        let mut buf = BytesMut::new();
        original.encode(&mut buf);
        let mut bytes = buf.freeze();

        let decoded = BjseBinary::decode(&mut bytes).unwrap();
        assert_eq!(original, decoded);
    }
}
