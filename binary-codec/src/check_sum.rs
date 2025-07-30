use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use bytes::BytesMut;
use crc::{CRC_32_ISO_HDLC, Crc};
use once_cell::sync::Lazy;

pub enum Checksum {
    U8(u8),
    U16(u16),
    U32(u32),
    I8(i8),
    I16(i16),
    I32(i32),
}

pub trait ChecksumService: Send + Sync {
    fn algorithm(&self) -> &'static str;
    fn calc(&self, input: &BytesMut) -> Checksum;
}

pub struct Crc16ChecksumService;

impl ChecksumService for Crc16ChecksumService {
    fn algorithm(&self) -> &'static str {
        "CRC16"
    }

    fn calc(&self, input: &BytesMut) -> Checksum {
        let mut crc: u16 = 0xFFFF;
        for &b in input {
            crc ^= b as u16;
            for _ in 0..8 {
                if crc & 0x0001 != 0 {
                    crc = (crc >> 1) ^ 0xA001;
                } else {
                    crc >>= 1;
                }
            }
        }
        Checksum::U16(crc)
    }
}

pub struct Crc32ChecksumService;

impl ChecksumService for Crc32ChecksumService {
    fn algorithm(&self) -> &'static str {
        "CRC32"
    }

    fn calc(&self, input: &BytesMut) -> Checksum {
        let crc32 = Crc::<u32>::new(&CRC_32_ISO_HDLC);
        Checksum::U32(crc32.checksum(input))
    }
}

pub struct SsebinChecksumService;

impl ChecksumService for SsebinChecksumService {
    fn algorithm(&self) -> &'static str {
        "SSE_BIN"
    }

    fn calc(&self, input: &BytesMut) -> Checksum {
        let mut checksum: u32 = 0;
        for &b in input {
            checksum = (checksum + b as u32) & 0xFF;
        }
        Checksum::U32(checksum)
    }
}

pub struct SzsebinChecksumService;

impl ChecksumService for SzsebinChecksumService {
    fn algorithm(&self) -> &'static str {
        "SZSE_BIN"
    }

    fn calc(&self, input: &BytesMut) -> Checksum {
        let mut checksum: i32 = 0;
        for &b in input {
            checksum += b as i32
        }
        Checksum::I32(checksum % 256)
    }
}

pub struct ChecksumServiceContext {
    cache: RwLock<HashMap<&'static str, Arc<dyn ChecksumService>>>,
}

impl ChecksumServiceContext {
    pub fn register<T: ChecksumService + 'static>(&self, service: T) -> bool {
        let algo = service.algorithm();
        let mut cache = self.cache.write().unwrap();
        if cache.contains_key(algo) {
            false
        } else {
            cache.insert(algo, Arc::new(service));
            true
        }
    }

    pub fn get(&self, algorithm: &str) -> Option<Arc<dyn ChecksumService>> {
        let cache = self.cache.read().unwrap();
        cache.get(algorithm).cloned()
    }
}

pub static CHECKSUM_SERVICE_CONTEXT: Lazy<ChecksumServiceContext> = Lazy::new(|| {
    let ctx = ChecksumServiceContext {
        cache: RwLock::new(HashMap::new()),
    };
    ctx.register(Crc16ChecksumService);
    ctx.register(Crc32ChecksumService);
    ctx.register(SsebinChecksumService);
    ctx.register(SzsebinChecksumService);
    ctx
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crc16_checksum() {
        let svc = Crc16ChecksumService;
        let buf = BytesMut::from("123456789");
        let checksum = svc.calc(&buf);
        match checksum {
            Checksum::U16(val) => assert_eq!(val, 0x4B37, "CRC16 checksum mismatch"),
            _ => panic!("Unexpected checksum type"),
        }
    }

    #[test]
    fn test_crc16_empty() {
        let svc = Crc16ChecksumService;
        let buf = BytesMut::new();
        let checksum = svc.calc(&buf);

        match checksum {
            Checksum::U16(val) => {
                assert_eq!(val, 0xFFFF, "CRC16 of empty buffer should be initial value")
            }
            _ => panic!("Unexpected checksum type"),
        }
    }

    #[test]
    fn test_crc16_custom_data() {
        let svc = Crc16ChecksumService;
        let buf = BytesMut::from(&[0x01u8, 0x02, 0x03][..]);
        let checksum = svc.calc(&buf);
        match checksum {
            Checksum::U16(val) => {
                assert!(val > 0)
            }
            _ => panic!("Unexpected checksum type"),
        }
    }

    #[test]
    fn test_crc32_checksum() {
        let svc = Crc32ChecksumService;
        let buf = BytesMut::from("123456789");
        let checksum = svc.calc(&buf);

        match checksum {
            Checksum::U32(val) => {
                assert_eq!(val, 0xCBF43926, "CRC32 checksum mismatch")
            }
            _ => panic!("Unexpected checksum type"),
        }
    }

    #[test]
    fn test_crc32_empty() {
        let svc: Crc32ChecksumService = Crc32ChecksumService;
        let buf = BytesMut::new();
        let checksum = svc.calc(&buf);
        match checksum {
            Checksum::U32(val) => {
                assert_eq!(val, 0x00000000, "CRC32 of empty buffer should be 0")
            }
            _ => panic!("Unexpected checksum type"),
        }
    }

    #[test]
    fn test_crc32_custom_data() {
        let svc = Crc32ChecksumService;
        let buf = BytesMut::from(&[0x01u8, 0x02, 0x03][..]);
        let checksum = svc.calc(&buf);
        match checksum {
            Checksum::U32(val) => {
                assert!(val > 0)
            }
            _ => panic!("Unexpected checksum type"),
        }
    }
}
