use binary_codec::*;
use bytes::{Bytes, BytesMut};
use criterion::{Criterion, criterion_group, criterion_main};
use szse_binary::{
    extend_100101::Extend100101,
    new_order::{NewOrder, NewOrderApplExtendEnum},
    szse_binary::{SzseBinary, SzseBinaryBodyEnum},
};

fn bench_encode(c: &mut Criterion) {
    let msg = SzseBinary {
        msg_type: 1,
        body_length: 0,
        body: SzseBinaryBodyEnum::NewOrder(NewOrder {
            appl_id: "010".to_string(),
            submitting_pbuid: "b001".to_string(),
            security_id: "000001".to_string(),
            security_id_source: "101".to_string(),
            owner_type: 1,
            clearing_firm: "c01".to_string(),
            transact_time: 20250101123000,
            user_info: "u01".to_string(),
            cl_ord_id: "c00001".to_string(),
            account_id: "a01".to_string(),
            branch_id: "b01".to_string(),
            order_restrictions: "o01".to_string(),
            side: "1".to_string(),
            ord_type: "1".to_string(),
            order_qty: 1000,
            price: 90,
            appl_extend: NewOrderApplExtendEnum::Extend100101(Extend100101 {
                stop_px: 100,
                min_qty: 10,
                max_price_levels: 1,
                time_in_force: "1".to_string(),
                cash_margin: "1".to_string(),
            }),
        }),
        checksum: 0,
    };

    c.bench_function("encode_szse_binary", |b| {
        b.iter(|| {
            let mut buf = BytesMut::with_capacity(1024);
            msg.encode(&mut buf);
            buf
        })
    });
}

fn bench_decode(c: &mut Criterion) {
    let msg = SzseBinary {
        msg_type: 1,
        body_length: 0,
        body: SzseBinaryBodyEnum::NewOrder(NewOrder {
            appl_id: "010".to_string(),
            submitting_pbuid: "b001".to_string(),
            security_id: "000001".to_string(),
            security_id_source: "101".to_string(),
            owner_type: 1,
            clearing_firm: "c01".to_string(),
            transact_time: 20250101123000,
            user_info: "u01".to_string(),
            cl_ord_id: "c00001".to_string(),
            account_id: "a01".to_string(),
            branch_id: "b01".to_string(),
            order_restrictions: "o01".to_string(),
            side: "1".to_string(),
            ord_type: "1".to_string(),
            order_qty: 1000,
            price: 90,
            appl_extend: NewOrderApplExtendEnum::Extend100101(Extend100101 {
                stop_px: 100,
                min_qty: 10,
                max_price_levels: 1,
                time_in_force: "1".to_string(),
                cash_margin: "1".to_string(),
            }),
        }),
        checksum: 0,
    };
    let mut buf = BytesMut::with_capacity(1024);
    msg.encode(&mut buf);

    c.bench_function("decode_szse_binary", |b| {
        b.iter(|| {
            let mut bytes = Bytes::from(buf.clone());
            SzseBinary::decode(&mut bytes)
        })
    });
}

criterion_group!(benches, bench_encode, bench_decode);
criterion_main!(benches);
