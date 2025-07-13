use anyhow::Ok;
use binary_codec::BinaryCodec;
use bytes::BytesMut;
use sse_binary::exec_rpt_sync::{ExecRptSync, SubExecRptSync};
use sse_binary::logout::Logout;
use sse_binary::new_order_single::NewOrderSingle;
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
        body: SseBinaryBodyEnum::Logon(Logon {
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
        body: SseBinaryBodyEnum::Heartbeat(Heartbeat {}),
        checksum: 123456,
    };
    let _ = send_message(&mut stream, &heartbeat).await;

    let logout = SseBinary {
        msg_seq_num: 3,
        msg_body_len: 123456,
        msg_type: 41,
        body: SseBinaryBodyEnum::Logout(Logout {
            session_status: 1,
            text: "hello".to_string(),
        }),
        checksum: 123456,
    };
    let _ = send_message(&mut stream, &logout).await;

    let new_order_single = SseBinary {
        msg_seq_num: 3,
        msg_body_len: 123456,
        msg_type: 58,
        body: SseBinaryBodyEnum::NewOrderSingle(NewOrderSingle {
            biz_id: 1,
            biz_pbu: "2".to_string(),
            cl_ord_id: "3".to_string(),
            security_id: "4".to_string(),
            account: "5".to_string(),
            owner_type: 7,
            side: "8".to_string(),
            price: 9,
            order_qty: 10,
            ord_type: "1".to_string(),
            time_in_force: "2".to_string(),
            transact_time: 11,
            credit_tag: "13".to_string(),
            clearing_firm: "14".to_string(),
            branch_id: "15".to_string(),
            user_info: "16".to_string(),
        }),
        checksum: 123456,
    };
    let _ = send_message(&mut stream, &new_order_single).await;

    let sub_exec_rpt_syncs = vec![SubExecRptSync {
        pbu: "p1".to_string(),
        set_id: 1,
        begin_report_index: 2,
    }];

    let exec_rpt_sync = SseBinary {
        msg_seq_num: 3,
        msg_body_len: 123456,
        msg_type: 206,
        body: SseBinaryBodyEnum::ExecRptSync(ExecRptSync {
            sub_exec_rpt_sync: sub_exec_rpt_syncs,
        }),
        checksum: 123456,
    };
    let _ = send_message(&mut stream, &exec_rpt_sync).await;

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
