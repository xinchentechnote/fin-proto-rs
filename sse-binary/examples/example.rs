use std::io::Write;

use binary_codec::BinaryCodec;
use bytes::BytesMut;
use sse_binary::heartbeat::Heartbeat;
use sse_binary::sse_binary::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a sample message
    let original = SseBinary {
        msg_seq_num: 123456789,
        msg_body_len: 123456,
        msg_type: 33,
        body: SseBinaryBodyEnum::Heartbeat(Heartbeat {}),
        checksum: 123456,
    };

    println!("Original: {:?}", original);

    // Write to file
    let filename = "message.bin";
    let mut buf = BytesMut::new();
    original.encode(&mut buf);
    let bytes = buf.freeze();

    let mut file = std::fs::File::create(filename)?;
    file.write_all(&bytes)?;
    Ok(())
}
