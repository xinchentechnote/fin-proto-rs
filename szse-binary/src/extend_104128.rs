// Code generated by fin-protoc. DO NOT EDIT.
use binary_codec::*;
use bytes::{Buf, BufMut, Bytes, BytesMut};

#[derive(Debug, Clone, PartialEq)]
pub struct Extend104128 {
    pub member_id: String,
    pub investor_type: String,
    pub investor_id: String,
    pub investor_name: String,
    pub trader_code: String,
    pub secondary_order_id: String,
    pub bid_trans_type: u16,
    pub bid_exec_inst_type: u16,
    pub low_limit_price: i64,
    pub high_limit_price: i64,
    pub min_qty: i64,
    pub trade_date: u32,
    pub settl_type: u16,
    pub settl_period: u8,
    pub pre_trade_anonymity: u8,
    pub cash_margin: String,
    pub memo: String,
}

impl BinaryCodec for Extend104128 {
    fn encode(&self, buf: &mut BytesMut) {
        put_char_array(buf, &self.member_id, 6);
        put_char_array(buf, &self.investor_type, 2);
        put_char_array(buf, &self.investor_id, 10);
        put_char_array(buf, &self.investor_name, 120);
        put_char_array(buf, &self.trader_code, 8);
        put_char_array(buf, &self.secondary_order_id, 16);
        buf.put_u16(self.bid_trans_type);
        buf.put_u16(self.bid_exec_inst_type);
        buf.put_i64(self.low_limit_price);
        buf.put_i64(self.high_limit_price);
        buf.put_i64(self.min_qty);
        buf.put_u32(self.trade_date);
        buf.put_u16(self.settl_type);
        buf.put_u8(self.settl_period);
        buf.put_u8(self.pre_trade_anonymity);
        put_char_array(buf, &self.cash_margin, 1);
        put_char_array(buf, &self.memo, 160);
    }

    fn decode(buf: &mut Bytes) -> Option<Extend104128> {
        let member_id = get_char_array(buf, 6)?;
        let investor_type = get_char_array(buf, 2)?;
        let investor_id = get_char_array(buf, 10)?;
        let investor_name = get_char_array(buf, 120)?;
        let trader_code = get_char_array(buf, 8)?;
        let secondary_order_id = get_char_array(buf, 16)?;
        let bid_trans_type = buf.get_u16();
        let bid_exec_inst_type = buf.get_u16();
        let low_limit_price = buf.get_i64();
        let high_limit_price = buf.get_i64();
        let min_qty = buf.get_i64();
        let trade_date = buf.get_u32();
        let settl_type = buf.get_u16();
        let settl_period = buf.get_u8();
        let pre_trade_anonymity = buf.get_u8();
        let cash_margin = get_char_array(buf, 1)?;
        let memo = get_char_array(buf, 160)?;
        Some(Self {
            member_id,
            investor_type,
            investor_id,
            investor_name,
            trader_code,
            secondary_order_id,
            bid_trans_type,
            bid_exec_inst_type,
            low_limit_price,
            high_limit_price,
            min_qty,
            trade_date,
            settl_type,
            settl_period,
            pre_trade_anonymity,
            cash_margin,
            memo,
        })
    }
}

#[cfg(test)]
mod extend_104128_tests {
    use super::*;
    use bytes::BytesMut;

    #[test]
    fn test_extend_104128_codec() {
        let original = Extend104128 {
            member_id: vec!['a'; 6].into_iter().collect::<String>(),
            investor_type: vec!['a'; 2].into_iter().collect::<String>(),
            investor_id: vec!['a'; 10].into_iter().collect::<String>(),
            investor_name: vec!['a'; 120].into_iter().collect::<String>(),
            trader_code: vec!['a'; 8].into_iter().collect::<String>(),
            secondary_order_id: vec!['a'; 16].into_iter().collect::<String>(),
            bid_trans_type: 1234,
            bid_exec_inst_type: 1234,
            low_limit_price: -123456789,
            high_limit_price: -123456789,
            min_qty: -123456789,
            trade_date: 123456,
            settl_type: 1234,
            settl_period: 42,
            pre_trade_anonymity: 42,
            cash_margin: vec!['a'; 1].into_iter().collect::<String>(),
            memo: vec!['a'; 160].into_iter().collect::<String>(),
        };

        let mut buf = BytesMut::new();
        original.encode(&mut buf);
        let mut bytes = buf.freeze();

        let decoded = Extend104128::decode(&mut bytes).unwrap();
        assert_eq!(original, decoded);
    }
}
