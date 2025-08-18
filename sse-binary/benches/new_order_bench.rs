use binary_codec::*;
use bytes::BytesMut;
use criterion::{Criterion, criterion_group, criterion_main};
use sse_binary::{
    new_order_single::NewOrderSingle,
    sse_binary::{SseBinary, SseBinaryBodyEnum},
};

fn bench_encode(c: &mut Criterion) {
    let msg = NewOrderSingle {
        biz_id: 123456,
        biz_pbu: "PBUID001".to_string(),
        cl_ord_id: "ORD0001".to_string(),
        security_id: "600000".to_string(),
        account: "ACC123".to_string(),
        owner_type: 1,
        side: "1".to_string(),
        price: 12345,
        order_qty: 1000,
        ord_type: "2".to_string(),
        time_in_force: "0".to_string(),
        transact_time: 20250101120000,
        credit_tag: "CR".to_string(),
        clearing_firm: "CLR001".to_string(),
        branch_id: "BR001".to_string(),
        user_info: "TESTUSER".to_string(),
    };

    let wrapper = SseBinary {
        msg_type: 58,
        msg_seq_num: 1,
        msg_body_len: 0,
        body: SseBinaryBodyEnum::NewOrderSingle(msg),
        checksum: 0,
    };

    c.bench_function("encode_sse_binary", |b| {
        b.iter(|| {
            let mut buf = BytesMut::with_capacity(512);
            wrapper.encode(&mut buf);
            buf
        })
    });
}

fn bench_decode(c: &mut Criterion) {
    let msg = NewOrderSingle {
        biz_id: 123456,
        biz_pbu: "PBUID001".to_string(),
        cl_ord_id: "ORD0001".to_string(),
        security_id: "600000".to_string(),
        account: "ACC123".to_string(),
        owner_type: 1,
        side: "1".to_string(),
        price: 12345,
        order_qty: 1000,
        ord_type: "2".to_string(),
        time_in_force: "0".to_string(),
        transact_time: 20250101120000,
        credit_tag: "CR".to_string(),
        clearing_firm: "CLR001".to_string(),
        branch_id: "BR001".to_string(),
        user_info: "TESTUSER".to_string(),
    };

    let wrapper = SseBinary {
        msg_type: 58,
        msg_seq_num: 1,
        msg_body_len: 0,
        body: SseBinaryBodyEnum::NewOrderSingle(msg),
        checksum: 0,
    };

    let mut buf = BytesMut::with_capacity(512);
    wrapper.encode(&mut buf);
    let encoded = buf.freeze();

    c.bench_function("decode_sse_binary", |b| {
        b.iter(|| {
            let mut bytes = encoded.clone();
            SseBinary::decode(&mut bytes)
        })
    });
}

criterion_group!(benches, bench_encode, bench_decode);
criterion_main!(benches);
