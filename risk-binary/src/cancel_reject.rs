// Code generated by fin-protoc. DO NOT EDIT.
use binary_codec::*;
use bytes::{Buf, BufMut, Bytes, BytesMut};

#[derive(Debug, Clone, PartialEq)]
pub struct CancelReject {
    pub cl_ord_id: String,
    pub orig_cl_ord_id: String,
    pub cxl_rej_reason: u32,
}

impl BinaryCodec for CancelReject {
    fn encode(&self, buf: &mut BytesMut) {
        put_string::<u32>(buf, &self.cl_ord_id);
        put_string::<u32>(buf, &self.orig_cl_ord_id);
        buf.put_u32(self.cxl_rej_reason);
    }

    fn decode(buf: &mut Bytes) -> Option<CancelReject> {
        let cl_ord_id = get_string::<u32>(buf)?;
        let orig_cl_ord_id = get_string::<u32>(buf)?;
        let cxl_rej_reason = buf.get_u32();
        Some(Self {
            cl_ord_id,
            orig_cl_ord_id,
            cxl_rej_reason,
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
            cl_ord_id: "example".to_string(),
            orig_cl_ord_id: "example".to_string(),
            cxl_rej_reason: 123456,
        };

        let mut buf = BytesMut::new();
        original.encode(&mut buf);
        let mut bytes = buf.freeze();

        let decoded = CancelReject::decode(&mut bytes).unwrap();
        assert_eq!(original, decoded);
    }
}
