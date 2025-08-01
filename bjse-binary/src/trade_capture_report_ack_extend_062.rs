// Code generated by fin-protoc. DO NOT EDIT.
use binary_codec::*;
use bytes::{Bytes, BytesMut};

#[derive(Debug, Clone, PartialEq)]
pub struct TradeCaptureReportAckExtend062 {
    pub cash_margin: String,
}

impl BinaryCodec for TradeCaptureReportAckExtend062 {
    fn encode(&self, buf: &mut BytesMut) {
        put_char_array(buf, &self.cash_margin, 1);
    }

    fn decode(buf: &mut Bytes) -> Option<TradeCaptureReportAckExtend062> {
        let cash_margin = get_char_array(buf, 1)?;
        Some(Self { cash_margin })
    }
}

#[cfg(test)]
mod trade_capture_report_ack_extend_062_tests {
    use super::*;
    use bytes::BytesMut;

    #[test]
    fn test_trade_capture_report_ack_extend_062_codec() {
        let original = TradeCaptureReportAckExtend062 {
            cash_margin: vec!['a'; 1].into_iter().collect::<String>(),
        };

        let mut buf = BytesMut::new();
        original.encode(&mut buf);
        let mut bytes = buf.freeze();

        let decoded = TradeCaptureReportAckExtend062::decode(&mut bytes).unwrap();
        assert_eq!(original, decoded);
    }
}
