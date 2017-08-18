extern crate byteorder;

use self::byteorder::{BigEndian, WriteBytesExt};
use std::io::Result;
use std::marker::Sized;
use std::str::from_utf8;
use rfc1350::{ErrorCode, Opcode, TransferMode};

trait Serializable {
    fn serialize(self: Self) -> Vec<u8>;
    //fn deserialize(Vec<u8>) -> Result<Self> where Self: Sized;
}

struct RrqOperation {
    opcode: Opcode,
    filename: String,
    mode: TransferMode
}

impl Serializable for RrqOperation {
    fn serialize(self) -> Vec<u8> {
        let mut serialized = vec![];

        serialized.write_u16::<BigEndian>(self.opcode as u16);
        serialized.extend(self.filename.into_bytes());
        serialized.extend(self.mode.as_string().into_bytes());

        serialized
    }
}
/*
    fn deserialize(bytes: Vec<u8>) -> Result<RrqOperation> {
        let opcode : Opcode = bytes[0 .. 1];
        opcode = opcode.from_be();

        let filename_end : usize = bytes[1 ..].binary_search(0x00)?;
        let filename : &str = from_utf8(bytes[2 .. filename_end])?;
        let mode_end : usize = bytes[filename_end ..].binary_search(0x00)?;
        let mode : &str = from_utf8(bytes[filename_end .. mode_end])?;

        Ok(RrqOperation {
            opcode,
            filename,
            mode
        })
    }
}

struct WrqOperation {
    opcode: Opcode,
    filename: &'static str,
    mode: TransferMode
}

impl Serializable for WrqOperation {
    fn serialize(self) -> [u8] {}
    fn deserialize(bytes: [u8]) -> Result<WrqOperation> { }
}

struct DataOperation {
    opcode: Opcode,
    blocknum: u16,
    data: [u8]
}

impl Serializable for DataOperation {
    fn serialize(self) -> [u8] {}
    fn deserialize(bytes: [u8]) -> Result<DataOperation> {}
}

struct AckOperation {
    opcode: Opcode,
    blocknum: u16,
}

impl Serializable for AckOperation {
    fn serialize(self) -> [u8] {}
    fn deserialize(bytes: [u8]) -> Result<AckOperation> {}
}

struct ErrorOperation {
    opcode: Opcode,
    errorcode: ErrorCode,
    errormsg: &'static str
}

impl Serializable for ErrorOperation {
    fn serialize(self) -> [u8] {}
    fn deserialize(bytes: [u8]) -> Result<ErrorOperation> {}
}
*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rrq_serialization() {
        let operation = RrqOperation {
            opcode: Opcode::Rrq,
            filename: String::from("passwd"),
            mode: TransferMode::NetAscii
        };
        let expected_bytes = [
            0x00, 0x01, // 1 (big endian)
            0x70, 0x61, 0x73, 0x73, 0x77, 0x64, // p a s s w d
            0x6E, 0x65, 0x74, 0x61, 0x73, 0x63, 0x69, 0x69 // n e t a s c i i
        ];
        assert_eq!(operation.serialize(), expected_bytes);
    }
}
