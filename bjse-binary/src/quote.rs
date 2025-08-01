// Code generated by fin-protoc. DO NOT EDIT.
use binary_codec::*;
use bytes::{Buf, BufMut, Bytes, BytesMut};

use crate::quote_extend_070::*;
use crate::quote_extend_071::*;

#[derive(Debug, Clone, PartialEq)]
pub enum QuoteApplExtendEnum {
    QuoteExtend070(QuoteExtend070),
    QuoteExtend071(QuoteExtend071),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Quote {
    pub appl_id: String,
    pub submitting_pbuid: String,
    pub security_id: String,
    pub security_id_source: String,
    pub owner_type: u16,
    pub clearing_firm: String,
    pub transact_time: i64,
    pub user_info: String,
    pub quote_msg_id: String,
    pub account_id: String,
    pub quote_req_id: String,
    pub quote_type: u8,
    pub bid_px: i64,
    pub offer_px: i64,
    pub bid_size: i64,
    pub offer_size: i64,
    pub appl_extend: QuoteApplExtendEnum,
}

impl BinaryCodec for Quote {
    fn encode(&self, buf: &mut BytesMut) {
        put_char_array(buf, &self.appl_id, 3);
        put_char_array(buf, &self.submitting_pbuid, 6);
        put_char_array(buf, &self.security_id, 8);
        put_char_array(buf, &self.security_id_source, 4);
        buf.put_u16_le(self.owner_type);
        put_char_array(buf, &self.clearing_firm, 2);
        buf.put_i64_le(self.transact_time);
        put_char_array(buf, &self.user_info, 32);
        put_char_array(buf, &self.quote_msg_id, 10);
        put_char_array(buf, &self.account_id, 10);
        put_char_array(buf, &self.quote_req_id, 10);
        buf.put_u8(self.quote_type);
        buf.put_i64_le(self.bid_px);
        buf.put_i64_le(self.offer_px);
        buf.put_i64_le(self.bid_size);
        buf.put_i64_le(self.offer_size);
        match &self.appl_extend {
            QuoteApplExtendEnum::QuoteExtend070(msg) => msg.encode(buf),
            QuoteApplExtendEnum::QuoteExtend071(msg) => msg.encode(buf),
        }
    }

    fn decode(buf: &mut Bytes) -> Option<Quote> {
        let appl_id = get_char_array(buf, 3)?;
        let submitting_pbuid = get_char_array(buf, 6)?;
        let security_id = get_char_array(buf, 8)?;
        let security_id_source = get_char_array(buf, 4)?;
        let owner_type = buf.get_u16_le();
        let clearing_firm = get_char_array(buf, 2)?;
        let transact_time = buf.get_i64_le();
        let user_info = get_char_array(buf, 32)?;
        let quote_msg_id = get_char_array(buf, 10)?;
        let account_id = get_char_array(buf, 10)?;
        let quote_req_id = get_char_array(buf, 10)?;
        let quote_type = buf.get_u8();
        let bid_px = buf.get_i64_le();
        let offer_px = buf.get_i64_le();
        let bid_size = buf.get_i64_le();
        let offer_size = buf.get_i64_le();
        let appl_extend = match appl_id.as_str() {
            "070" => QuoteApplExtendEnum::QuoteExtend070(QuoteExtend070::decode(buf)?),
            "071" => QuoteApplExtendEnum::QuoteExtend071(QuoteExtend071::decode(buf)?),
            _ => return None,
        };
        Some(Self {
            appl_id,
            submitting_pbuid,
            security_id,
            security_id_source,
            owner_type,
            clearing_firm,
            transact_time,
            user_info,
            quote_msg_id,
            account_id,
            quote_req_id,
            quote_type,
            bid_px,
            offer_px,
            bid_size,
            offer_size,
            appl_extend,
        })
    }
}

#[cfg(test)]
mod quote_tests {
    use super::*;
    use bytes::BytesMut;

    #[test]
    fn test_quote_codec() {
        let original = Quote {
            submitting_pbuid: vec!['a'; 6].into_iter().collect::<String>(),
            security_id: vec!['a'; 8].into_iter().collect::<String>(),
            security_id_source: vec!['a'; 4].into_iter().collect::<String>(),
            owner_type: 1234,
            clearing_firm: vec!['a'; 2].into_iter().collect::<String>(),
            transact_time: -123456789,
            user_info: vec!['a'; 32].into_iter().collect::<String>(),
            quote_msg_id: vec!['a'; 10].into_iter().collect::<String>(),
            account_id: vec!['a'; 10].into_iter().collect::<String>(),
            quote_req_id: vec!['a'; 10].into_iter().collect::<String>(),
            quote_type: 42,
            bid_px: -123456789,
            offer_px: -123456789,
            bid_size: -123456789,
            offer_size: -123456789,
            appl_id: "070".to_string(),
            appl_extend: QuoteApplExtendEnum::QuoteExtend070(QuoteExtend070 {
                branch_id: vec!['a'; 2].into_iter().collect::<String>(),
                quote_id: vec!['a'; 10].into_iter().collect::<String>(),
                quote_resp_id: vec!['a'; 10].into_iter().collect::<String>(),
                private_quote: 42,
                valid_until_time: -123456789,
                price_type: 42,
                cash_margin: vec!['a'; 1].into_iter().collect::<String>(),
                counter_party_pbuid: vec!['a'; 6].into_iter().collect::<String>(),
                memo: vec!['a'; 120].into_iter().collect::<String>(),
            }),
        };

        let mut buf = BytesMut::new();
        original.encode(&mut buf);
        let mut bytes = buf.freeze();

        let decoded = Quote::decode(&mut bytes).unwrap();
        assert_eq!(original, decoded);
    }
}
