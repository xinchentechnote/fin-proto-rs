use binary_codec::BinaryCodec;
use binary_codec_macro::BinCodec;
use bytes::BytesMut;

#[derive(Debug, PartialEq, BinCodec)]
pub struct TestMessage {
    pub number: u16,
}

fn main() {
    let msg = TestMessage { number: 123 };

    let mut buf = BytesMut::new();
    msg.encode(&mut buf);

    let mut frozen = buf.freeze();
    let decoded = TestMessage::decode(&mut frozen).unwrap();

    println!("✅ Original: {:?}", msg);
    println!("✅ Decoded:  {:?}", decoded);
    assert_eq!(msg, decoded);
}
