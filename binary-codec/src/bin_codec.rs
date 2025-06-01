use bytes::{Buf, BufMut, Bytes, BytesMut};

pub trait BinaryCodec: Sized {
    fn encode(&self, buf: &mut BytesMut);
    fn decode(buf: &mut Bytes) -> Option<Self>;
}

// Utility Functions
pub fn put_string(buf: &mut BytesMut, s: &str) {
    buf.put_u16(s.len() as u16);
    buf.extend_from_slice(s.as_bytes());
}

pub fn get_string(buf: &mut Bytes) -> Option<String> {
    if buf.remaining() < 2 {
        return None;
    }
    let len = buf.get_u16() as usize;
    if buf.remaining() < len {
        return None;
    }
    Some(String::from_utf8(buf.copy_to_bytes(len).to_vec()).ok()?)
}

pub fn put_char(buf: &mut BytesMut, c: char) {
    buf.put_u8(c as u8);
}

pub fn get_char(buf: &mut Bytes) -> Option<char> {
    if buf.remaining() < 1 {
        return None;
    }
    Some(buf.get_u8() as char)
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::{Bytes, BytesMut};

    #[test]
    fn test_put_get_string() {
        let mut buf = BytesMut::new();
        let original = "Hello, Rust!";
        put_string(&mut buf, original);

        let mut bytes = buf.freeze();
        let decoded = get_string(&mut bytes);

        assert_eq!(decoded, Some(original.to_string()));
    }

    #[test]
    fn test_get_string_insufficient_length() {
        let mut buf = Bytes::from_static(&[0x00]); // less than 2 bytes for length
        assert_eq!(get_string(&mut buf), None);
    }

    #[test]
    fn test_get_string_incomplete_string() {
        let mut buf = Bytes::from_static(&[0x00, 0x05, b'H', b'e']); // len = 5, but only 2 bytes
        assert_eq!(get_string(&mut buf), None);
    }

    #[test]
    fn test_put_get_char() {
        let mut buf = BytesMut::new();
        let original = 'A';
        put_char(&mut buf, original);

        let mut bytes = buf.freeze();
        let decoded = get_char(&mut bytes);

        assert_eq!(decoded, Some(original));
    }

    #[test]
    fn test_get_char_empty() {
        let mut buf = Bytes::new(); // empty buffer
        assert_eq!(get_char(&mut buf), None);
    }

    #[test]
    fn test_binary_codec_trait_manual_impl() {
        #[derive(Debug, PartialEq)]
        struct MyMessage {
            msg: String,
        }

        impl BinaryCodec for MyMessage {
            fn encode(&self, buf: &mut BytesMut) {
                put_string(buf, &self.msg);
            }

            fn decode(buf: &mut Bytes) -> Option<Self> {
                let msg = get_string(buf)?;
                Some(MyMessage { msg })
            }
        }

        let original = MyMessage {
            msg: "Hello Codec".to_string(),
        };

        let mut buf = BytesMut::new();
        original.encode(&mut buf);

        let mut bytes = buf.freeze();
        let decoded = MyMessage::decode(&mut bytes);

        assert_eq!(decoded, Some(original));
    }
}
