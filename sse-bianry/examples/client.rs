use anyhow::Ok;
use binary_codec::BinaryCodec;
use bytes::BytesMut;
use sse_binary::sse_binary::*;
use sse_binary::{heartbeat::Heartbeat, logon::Logon};
use tokio::{io::AsyncWriteExt, net::TcpStream};

async fn send_message(stream: &mut TcpStream, original: &SseBinary) -> anyhow::Result<()> {
    let mut buf = BytesMut::new();
    original.encode(&mut buf);
    let bytes = buf.freeze();
    stream.write_all(&bytes).await?;
    println!("Sent message: {:?}", original);
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let connect = TcpStream::connect("127.0.0.1:8080").await?;
    let mut stream = connect;
    println!("Connected to server");

    let heartbeat = SseBinary {
        msg_seq_num: 1,
        msg_body_len: 123456,
        msg_type: 40,
        msg_type_body: SseBinaryMsgTypeEnum::Logon(Logon {
            sender_comp_id: "sender".to_string(),
            target_comp_id: "target".to_string(),
            heart_bt_int: 12,
            prtcl_version: "1.0".to_string(),
            trade_date: 20250605,
            q_size: 9,
        }),
        checksum: 123456,
    };
    let _ = send_message(&mut stream, &heartbeat).await;

    let heartbeat = SseBinary {
        msg_seq_num: 2,
        msg_body_len: 123456,
        msg_type: 33,
        msg_type_body: SseBinaryMsgTypeEnum::Heartbeat(Heartbeat {}),
        checksum: 123456,
    };
    let _ = send_message(&mut stream, &heartbeat).await;

    // 持续读取服务器响应
    // let mut buf = vec![0; 1024];
    // loop {
    //     match stream.read_buf(&mut buf).await {
    //         Ok(0) => {
    //             println!("Connection closed by server");
    //             break;
    //         }
    //         Ok(n) => {
    //             println!("Received {} bytes: {:?}", n, &buf[..n]);
    //         }
    //         Err(e) => {
    //             eprintln!("Failed to read from socket: {}", e);
    //             break;
    //         }
    //     }
    // }

    Ok(())
}
