use bytes::{Buf, BufMut, Bytes, BytesMut};

pub trait BinaryCodec: Sized {
    fn encode(&self, buf: &mut BytesMut);
    fn decode(buf: &mut Bytes) -> Option<Self>;
}

pub trait BasicTypeCodec: Sized {
    fn encode(&self, buf: &mut BytesMut, little_endian: bool);
    fn decode(buf: &mut Bytes, little_endian: bool) -> Option<Self>;
}

pub trait LengthPrefix: Sized {
    fn read(buf: &mut Bytes, little_endian: bool) -> Option<usize>;
    fn write(len: usize, buf: &mut BytesMut, little_endian: bool);
}

impl LengthPrefix for u8 {
    fn read(buf: &mut Bytes, _little_endian: bool) -> Option<usize> {
        if buf.remaining() >= 1 {
            Some(buf.get_u8() as usize)
        } else {
            None
        }
    }
    fn write(len: usize, buf: &mut BytesMut, _little_endian: bool) {
        buf.put_u8(len as u8);
    }
}

impl LengthPrefix for u16 {
    fn read(buf: &mut Bytes, little_endian: bool) -> Option<usize> {
        if buf.remaining() >= 2 {
            if little_endian {
                Some(buf.get_u16_le() as usize)
            } else {
                Some(buf.get_u16() as usize)
            }
        } else {
            None
        }
    }
    fn write(len: usize, buf: &mut BytesMut, little_endian: bool) {
        if little_endian {
            buf.put_u16_le(len as u16);
        } else {
            buf.put_u16(len as u16);
        }
    }
}

impl LengthPrefix for u32 {
    fn read(buf: &mut Bytes, little_endian: bool) -> Option<usize> {
        if buf.remaining() >= 4 {
            if little_endian {
                Some(buf.get_u32_le() as usize)
            } else {
                Some(buf.get_u32() as usize)
            }
        } else {
            None
        }
    }
    fn write(len: usize, buf: &mut BytesMut, little_endian: bool) {
        if little_endian {
            buf.put_u32_le(len as u32);
        } else {
            buf.put_u32(len as u32);
        }
    }
}

// Implement BasicTypeCodec for numeric types with endianness support
macro_rules! impl_basic_type_codec_numeric {
    ($type:ty, $put:ident, $put_le:ident, $get:ident, $get_le:ident) => {
        impl BasicTypeCodec for $type {
            fn encode(&self, buf: &mut BytesMut, little_endian: bool) {
                if little_endian {
                    buf.$put_le(*self);
                } else {
                    buf.$put(*self);
                }
            }
            fn decode(buf: &mut Bytes, little_endian: bool) -> Option<Self> {
                if buf.remaining() >= std::mem::size_of::<$type>() {
                    if little_endian {
                        Some(buf.$get_le())
                    } else {
                        Some(buf.$get())
                    }
                } else {
                    None
                }
            }
        }
    };
}

impl_basic_type_codec_numeric!(u8, put_u8, put_u8, get_u8, get_u8);
impl_basic_type_codec_numeric!(u16, put_u16, put_u16_le, get_u16, get_u16_le);
impl_basic_type_codec_numeric!(u32, put_u32, put_u32_le, get_u32, get_u32_le);
impl_basic_type_codec_numeric!(u64, put_u64, put_u64_le, get_u64, get_u64_le);
impl_basic_type_codec_numeric!(i8, put_i8, put_i8, get_i8, get_i8);
impl_basic_type_codec_numeric!(i16, put_i16, put_i16_le, get_i16, get_i16_le);
impl_basic_type_codec_numeric!(i32, put_i32, put_i32_le, get_i32, get_i32_le);
impl_basic_type_codec_numeric!(i64, put_i64, put_i64_le, get_i64, get_i64_le);
// impl_basic_type_codec_numeric!(char, put_u8, put_u8, get_u8, get_u8);
impl_basic_type_codec_numeric!(f32, put_f32, put_f32_le, get_f32, get_f32_le);
impl_basic_type_codec_numeric!(f64, put_f64, put_f64_le, get_f64, get_f64_le);

pub fn get_list<T, L>(buf: &mut Bytes) -> Option<Vec<T>>
where
    T: BasicTypeCodec,
    L: LengthPrefix,
{
    let len = L::read(buf, false)?;
    let mut result = Vec::with_capacity(len);
    for _ in 0..len {
        result.push(T::decode(buf, false)?);
    }
    Some(result)
}

pub fn get_list_le<T, L>(buf: &mut Bytes) -> Option<Vec<T>>
where
    T: BasicTypeCodec,
    L: LengthPrefix,
{
    let len = L::read(buf, true)?;
    let mut result = Vec::with_capacity(len);
    for _ in 0..len {
        result.push(T::decode(buf, true)?);
    }
    Some(result)
}

pub fn get_object_list<T, L>(buf: &mut Bytes) -> Option<Vec<T>>
where
    T: BinaryCodec,
    L: LengthPrefix,
{
    let len = L::read(buf, false)?;
    let mut result = Vec::with_capacity(len);
    for _ in 0..len {
        result.push(T::decode(buf)?);
    }
    Some(result)
}

pub fn get_object_list_le<T, L>(buf: &mut Bytes) -> Option<Vec<T>>
where
    T: BinaryCodec,
    L: LengthPrefix,
{
    let len = L::read(buf, true)?;
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
    let len = L1::read(buf, false)?;
    let mut result = Vec::with_capacity(len);
    for _ in 0..len {
        let str_len = L2::read(buf, false)?;
        if buf.remaining() < str_len {
            return None;
        }
        let bytes = buf.copy_to_bytes(str_len);
        let s = String::from_utf8(bytes.to_vec()).ok()?;
        result.push(s);
    }
    Some(result)
}

pub fn get_string_list_le<L1, L2>(buf: &mut Bytes) -> Option<Vec<String>>
where
    L1: LengthPrefix,
    L2: LengthPrefix,
{
    let len = L1::read(buf, true)?;
    let mut result = Vec::with_capacity(len);
    for _ in 0..len {
        let str_len = L2::read(buf, true)?;
        if buf.remaining() < str_len {
            return None;
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
    let len = L::read(buf, false)?;
    let mut result = Vec::with_capacity(len);
    for _ in 0..len {
        if buf.remaining() < fixed_length {
            return None;
        }
        let bytes = buf.copy_to_bytes(fixed_length);
        let s = String::from_utf8(bytes.to_vec()).ok()?;
        result.push(s.trim_end_matches('\0').to_string());
    }
    Some(result)
}

pub fn get_fixed_string_list_le<L>(buf: &mut Bytes, fixed_length: usize) -> Option<Vec<String>>
where
    L: LengthPrefix,
{
    let len = L::read(buf, true)?;
    let mut result = Vec::with_capacity(len);
    for _ in 0..len {
        if buf.remaining() < fixed_length {
            return None;
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
    L1::write(items.len(), buf, false);
    for item in items {
        let bytes = item.as_bytes();
        L2::write(bytes.len(), buf, false);
        buf.extend_from_slice(bytes);
    }
}

pub fn put_string_list_le<L1, L2>(buf: &mut BytesMut, items: &[String])
where
    L1: LengthPrefix,
    L2: LengthPrefix,
{
    L1::write(items.len(), buf, true);
    for item in items {
        let bytes = item.as_bytes();
        L2::write(bytes.len(), buf, true);
        buf.extend_from_slice(bytes);
    }
}

pub fn put_fixed_string_list<L>(buf: &mut BytesMut, items: &[String], fixed_length: usize)
where
    L: LengthPrefix,
{
    L::write(items.len(), buf, false);
    for item in items {
        let bytes = item.as_bytes();
        let len = bytes.len().min(fixed_length);
        buf.extend_from_slice(&bytes[..len]);
        if len < fixed_length {
            buf.extend_from_slice(&vec![0; fixed_length - len]);
        }
    }
}

pub fn put_fixed_string_list_le<L>(buf: &mut BytesMut, items: &[String], fixed_length: usize)
where
    L: LengthPrefix,
{
    L::write(items.len(), buf, true);
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
    T: BasicTypeCodec,
    L: LengthPrefix,
{
    L::write(items.len(), buf, false);
    for item in items {
        item.encode(buf, false);
    }
}

pub fn put_list_le<T, L>(buf: &mut BytesMut, items: &[T])
where
    T: BasicTypeCodec,
    L: LengthPrefix,
{
    L::write(items.len(), buf, true);
    for item in items {
        item.encode(buf, true);
    }
}

pub fn put_object_list<T, L>(buf: &mut BytesMut, items: &[T])
where
    T: BinaryCodec,
    L: LengthPrefix,
{
    L::write(items.len(), buf, false);
    for item in items {
        item.encode(buf);
    }
}

pub fn put_object_list_le<T, L>(buf: &mut BytesMut, items: &[T])
where
    T: BinaryCodec,
    L: LengthPrefix,
{
    L::write(items.len(), buf, true);
    for item in items {
        item.encode(buf);
    }
}

pub fn put_string<L>(buf: &mut BytesMut, s: &str)
where
    L: LengthPrefix,
{
    L::write(s.len(), buf, false);
    buf.extend_from_slice(s.as_bytes());
}

pub fn put_string_le<L>(buf: &mut BytesMut, s: &str)
where
    L: LengthPrefix,
{
    L::write(s.len(), buf, true);
    buf.extend_from_slice(s.as_bytes());
}

pub fn get_string<L>(buf: &mut Bytes) -> Option<String>
where
    L: LengthPrefix,
{
    let len = L::read(buf, false)?;
    if buf.remaining() < len {
        return None;
    }
    String::from_utf8(buf.copy_to_bytes(len).to_vec()).ok()
}

pub fn get_string_le<L>(buf: &mut Bytes) -> Option<String>
where
    L: LengthPrefix,
{
    let len = L::read(buf, true)?;
    if buf.remaining() < len {
        return None;
    }
    String::from_utf8(buf.copy_to_bytes(len).to_vec()).ok()
}

pub fn put_char_array_with_padding(
    buf: &mut BytesMut,
    s: &str,
    fixed_length: usize,
    padding: char,
    left: bool,
) {
    let bytes = s.as_bytes();
    let len = bytes.len().min(fixed_length);
    if left && fixed_length - len > 0 {
        buf.extend_from_slice(&vec![padding as u8; fixed_length - len]);
    }
    buf.extend_from_slice(&bytes[..len]);
    if !left && fixed_length - len > 0 {
        buf.extend_from_slice(&vec![padding as u8; fixed_length - len]);
    }
}

pub fn put_char_array(buf: &mut BytesMut, s: &str, fixed_length: usize) {
    put_char_array_with_padding(buf, s, fixed_length, ' ', false);
}

pub fn get_char_array_trim_padding(
    buf: &mut Bytes,
    len: usize,
    padding: char,
    left: bool,
) -> Option<String> {
    if buf.remaining() < len {
        return None;
    }
    let bytes = buf.copy_to_bytes(len);
    let s = String::from_utf8(bytes.to_vec()).ok()?;
    if left {
        Some(s.trim_start_matches(padding).to_string())
    } else {
        Some(s.trim_end_matches(padding).to_string())
    }
}

pub fn get_char_array(buf: &mut Bytes, len: usize) -> Option<String> {
    get_char_array_trim_padding(buf, len, ' ', false)
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
        put_string::<u8>(&mut buf, original);

        let mut bytes = buf.freeze();
        let decoded = get_string::<u8>(&mut bytes);

        assert_eq!(decoded, Some(original.to_string()));
    }

    #[test]
    fn test_get_string_insufficient_length() {
        let mut buf = Bytes::from_static(&[0x00]); // less than 2 bytes for length
        assert_eq!(get_string::<u16>(&mut buf), None);
    }

    #[test]
    fn test_get_string_incomplete_string() {
        let mut buf = Bytes::from_static(&[0x00, 0x05, b'H', b'e']); // len = 5, but only 2 bytes
        assert_eq!(get_string::<u16>(&mut buf), None);
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
                put_string::<u32>(buf, &self.msg);
            }

            fn decode(buf: &mut Bytes) -> Option<Self> {
                let msg = get_string::<u32>(buf)?;
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
