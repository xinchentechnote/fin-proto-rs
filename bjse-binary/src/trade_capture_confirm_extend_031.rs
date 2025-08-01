// Code generated by fin-protoc. DO NOT EDIT.
use binary_codec::*;
use bytes::{Bytes, BytesMut};

#[derive(Debug, Clone, PartialEq)]
pub struct TradeCaptureConfirmExtend031 {
    pub member_id: String,
    pub trader_code: String,
    pub counter_party_member_id: String,
    pub counter_party_trader_code: String,
    pub settl_type: String,
    pub settl_period: String,
    pub cash_margin: String,
    pub memo: String,
}

impl BinaryCodec for TradeCaptureConfirmExtend031 {
    fn encode(&self, buf: &mut BytesMut) {
        put_char_array(buf, &self.member_id, 6);
        put_char_array(buf, &self.trader_code, 5);
        put_char_array(buf, &self.counter_party_member_id, 6);
        put_char_array(buf, &self.counter_party_trader_code, 5);
        put_char_array(buf, &self.settl_type, 1);
        put_char_array(buf, &self.settl_period, 1);
        put_char_array(buf, &self.cash_margin, 1);
        put_char_array(buf, &self.memo, 120);
    }

    fn decode(buf: &mut Bytes) -> Option<TradeCaptureConfirmExtend031> {
        let member_id = get_char_array(buf, 6)?;
        let trader_code = get_char_array(buf, 5)?;
        let counter_party_member_id = get_char_array(buf, 6)?;
        let counter_party_trader_code = get_char_array(buf, 5)?;
        let settl_type = get_char_array(buf, 1)?;
        let settl_period = get_char_array(buf, 1)?;
        let cash_margin = get_char_array(buf, 1)?;
        let memo = get_char_array(buf, 120)?;
        Some(Self {
            member_id,
            trader_code,
            counter_party_member_id,
            counter_party_trader_code,
            settl_type,
            settl_period,
            cash_margin,
            memo,
        })
    }
}

#[cfg(test)]
mod trade_capture_confirm_extend_031_tests {
    use super::*;
    use bytes::BytesMut;

    #[test]
    fn test_trade_capture_confirm_extend_031_codec() {
        let original = TradeCaptureConfirmExtend031 {
            member_id: vec!['a'; 6].into_iter().collect::<String>(),
            trader_code: vec!['a'; 5].into_iter().collect::<String>(),
            counter_party_member_id: vec!['a'; 6].into_iter().collect::<String>(),
            counter_party_trader_code: vec!['a'; 5].into_iter().collect::<String>(),
            settl_type: vec!['a'; 1].into_iter().collect::<String>(),
            settl_period: vec!['a'; 1].into_iter().collect::<String>(),
            cash_margin: vec!['a'; 1].into_iter().collect::<String>(),
            memo: vec!['a'; 120].into_iter().collect::<String>(),
        };

        let mut buf = BytesMut::new();
        original.encode(&mut buf);
        let mut bytes = buf.freeze();

        let decoded = TradeCaptureConfirmExtend031::decode(&mut bytes).unwrap();
        assert_eq!(original, decoded);
    }
}
