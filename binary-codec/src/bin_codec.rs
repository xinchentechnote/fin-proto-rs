use bytes::{Buf, BufMut, Bytes, BytesMut};

pub trait BinaryCodec: Sized {
    fn encode(&self, buf: &mut BytesMut);
    fn decode(buf: &mut Bytes) -> Option<Self>;
}

pub trait LengthPrefix: Sized {
    fn read(buf: &mut Bytes) -> Option<usize>;
    fn write(len: usize, buf: &mut BytesMut);
}

impl LengthPrefix for u8 {
    fn read(buf: &mut Bytes) -> Option<usize> {
        if buf.remaining() >= 1 {
            Some(buf.get_u8() as usize)
        } else {
            None
        }
    }
    fn write(len: usize, buf: &mut BytesMut) {
        buf.put_u8(len as u8);
    }
}

impl LengthPrefix for u16 {
    fn read(buf: &mut Bytes) -> Option<usize> {
        if buf.remaining() >= 2 {
            Some(buf.get_u16() as usize)
        } else {
            None
        }
    }
    fn write(len: usize, buf: &mut BytesMut) {
        buf.put_u16(len as u16);
    }
}

impl LengthPrefix for u32 {
    fn read(buf: &mut Bytes) -> Option<usize> {
        if buf.remaining() >= 4 {
            Some(buf.get_u32() as usize)
        } else {
            None
        }
    }
    fn write(len: usize, buf: &mut BytesMut) {
        buf.put_u32(len as u32);
    }
}

impl BinaryCodec for u8 {
    fn encode(&self, buf: &mut BytesMut) {
        buf.put_u8(*self);
    }
    fn decode(buf: &mut Bytes) -> Option<Self> {
        if buf.remaining() >= 1 {
            Some(buf.get_u8())
        } else {
            None
        }
    }
}

impl BinaryCodec for u16 {
    fn encode(&self, buf: &mut BytesMut) {
        buf.put_u16(*self);
    }
    fn decode(buf: &mut Bytes) -> Option<Self> {
        if buf.remaining() >= 2 {
            Some(buf.get_u16())
        } else {
            None
        }
    }
}
impl BinaryCodec for u32 {
    fn encode(&self, buf: &mut BytesMut) {
        buf.put_u32(*self);
    }
    fn decode(buf: &mut Bytes) -> Option<Self> {
        if buf.remaining() >= 4 {
            Some(buf.get_u32())
        } else {
            None
        }
    }
}
impl BinaryCodec for u64 {
    fn encode(&self, buf: &mut BytesMut) {
        buf.put_u64(*self);
    }
    fn decode(buf: &mut Bytes) -> Option<Self> {
        if buf.remaining() >= 8 {
            Some(buf.get_u64())
        } else {
            None
        }
    }
}

impl BinaryCodec for i8 {
    fn encode(&self, buf: &mut BytesMut) {
        buf.put_i8(*self);
    }
    fn decode(buf: &mut Bytes) -> Option<Self> {
        if buf.remaining() >= 1 {
            Some(buf.get_i8())
        } else {
            None
        }
    }
}

impl BinaryCodec for i16 {
    fn encode(&self, buf: &mut BytesMut) {
        buf.put_i16(*self);
    }
    fn decode(buf: &mut Bytes) -> Option<Self> {
        if buf.remaining() >= 2 {
            Some(buf.get_i16())
        } else {
            None
        }
    }
}

impl BinaryCodec for i32 {
    fn encode(&self, buf: &mut BytesMut) {
        buf.put_i32(*self);
    }
    fn decode(buf: &mut Bytes) -> Option<Self> {
        if buf.remaining() >= 4 {
            Some(buf.get_i32())
        } else {
            None
        }
    }
}

impl BinaryCodec for i64 {
    fn encode(&self, buf: &mut BytesMut) {
        buf.put_i64(*self);
    }
    fn decode(buf: &mut Bytes) -> Option<Self> {
        if buf.remaining() >= 8 {
            Some(buf.get_i64())
        } else {
            None
        }
    }
}

impl BinaryCodec for String {
    fn encode(&self, buf: &mut BytesMut) {
        put_string(buf, self);
    }

    fn decode(buf: &mut Bytes) -> Option<Self> {
        get_string(buf)
    }
}

pub fn get_list<T, L>(buf: &mut Bytes) -> Option<Vec<T>>
where
    T: BinaryCodec,
    L: LengthPrefix,
{
    let len = L::read(buf)?;
    let mut result = Vec::with_capacity(len);
    for _ in 0..len {
        result.push(T::decode(buf)?);
    }
    Some(result)
}

pub fn get_string_list<L1, L2>(buf: &mut Bytes) -> Option<Vec<String>>
where
    L1: LengthPrefix,
    L2: LengthPrefix,
{
    let len = L1::read(buf)?;
    let mut result = Vec::with_capacity(len);
    for _ in 0..len {
        let str_len = L2::read(buf)?;
        if buf.remaining() < str_len {
            return None; // Not enough bytes for the string
        }
        let bytes = buf.copy_to_bytes(str_len);
        let s = String::from_utf8(bytes.to_vec()).ok()?;
        result.push(s);
    }
    Some(result)
}

pub fn get_fixed_string_list<L>(buf: &mut Bytes, fixed_length: usize) -> Option<Vec<String>>
where
    L: LengthPrefix,
{
    let len = L::read(buf)?;
    let mut result = Vec::with_capacity(len);
    for _ in 0..len {
        if buf.remaining() < fixed_length {
            return None; // Not enough bytes for the fixed-length string
        }
        let bytes = buf.copy_to_bytes(fixed_length);
        let s = String::from_utf8(bytes.to_vec()).ok()?;
        result.push(s.trim_end_matches('\0').to_string());
    }
    Some(result)
}

pub fn put_string_list<L1, L2>(buf: &mut BytesMut, items: &[String])
where
    L1: LengthPrefix,
    L2: LengthPrefix,
{
    L1::write(items.len(), buf);
    for item in items {
        let bytes = item.as_bytes();
        L2::write(bytes.len(), buf);
        buf.extend_from_slice(bytes);
    }
}

pub fn put_fixed_string_list<L>(buf: &mut BytesMut, items: &[String], fixed_length: usize)
where
    L: LengthPrefix,
{
    L::write(items.len(), buf);
    for item in items {
        let bytes = item.as_bytes();
        let len = bytes.len().min(fixed_length);
        buf.extend_from_slice(&bytes[..len]);
        if len < fixed_length {
            buf.extend_from_slice(&vec![0; fixed_length - len]);
        }
    }
}

pub fn put_list<T, L>(buf: &mut BytesMut, items: &[T])
where
    T: BinaryCodec,
    L: LengthPrefix,
{
    L::write(items.len(), buf);
    for item in items {
        item.encode(buf);
    }
}

// Utility Functions
pub fn put_string(buf: &mut BytesMut, s: &str) {
    buf.put_u16(s.len() as u16);
    buf.extend_from_slice(s.as_bytes());
}

// put_char_array writes a fixed-length string to the buffer, padding with zeros if necessary.
pub fn put_char_array(buf: &mut BytesMut, s: &str, fixed_length: usize) {
    let bytes = s.as_bytes();
    let len = bytes.len().min(fixed_length);
    buf.extend_from_slice(&bytes[..len]);
    if len < fixed_length {
        buf.extend_from_slice(&vec![0; fixed_length - len]);
    }
}

// get_char_array retrieves a fixed-length string from the buffer.
pub fn get_char_array(buf: &mut Bytes, fixed_length: usize) -> Option<String> {
    if buf.remaining() < fixed_length {
        return None;
    }
    let bytes = buf.copy_to_bytes(fixed_length);
    let s = String::from_utf8(bytes.to_vec()).ok()?;
    Some(s.trim_end_matches('\0').to_string())
}

pub fn get_string(buf: &mut Bytes) -> Option<String> {
    if buf.remaining() < 2 {
        return None;
    }
    let len = buf.get_u16() as usize;
    if buf.remaining() < len {
        return None;
    }
    String::from_utf8(buf.copy_to_bytes(len).to_vec()).ok()
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

    #[test]
    fn test_put_string_fixed() {
        let mut buf = BytesMut::new();
        let original = "Hello";
        put_char_array(&mut buf, original, 10);

        let mut bytes = buf.freeze();
        let decoded = get_char_array(&mut bytes, 10);

        assert_eq!(decoded, Some("Hello".to_string()));
    }

    #[test]
    fn test_put_list() {
        let mut buf = BytesMut::new();
        let original = vec![1, 2, 3];
        put_list::<u8, u16>(&mut buf, &original);

        let mut bytes = buf.freeze();
        let decoded = get_list::<u8, u16>(&mut bytes);

        assert_eq!(decoded, Some(original));
    }

    #[test]
    fn test_put_string_list() {
        let mut buf = BytesMut::new();
        let original = vec!["Hello".to_string(), "World".to_string()];
        put_string_list::<u16, u16>(&mut buf, &original);

        let mut bytes = buf.freeze();
        let decoded = get_string_list::<u16, u16>(&mut bytes);

        assert_eq!(decoded, Some(original));
    }

    #[test]
    fn test_put_fixed_string_list() {
        let mut buf = BytesMut::new();
        let original = vec!["Hello".to_string(), "World".to_string()];
        put_fixed_string_list::<u16>(&mut buf, &original, 10);

        let mut bytes = buf.freeze();
        let decoded = get_fixed_string_list::<u16>(&mut bytes, 10);

        assert_eq!(
            decoded,
            Some(vec!["Hello".to_string(), "World".to_string()])
        );
    }
}
