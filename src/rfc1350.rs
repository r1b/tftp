// https://tools.ietf.org/html/rfc1350

pub const BLOCK_LENGTH: u16 = 512;
pub const REQUESTING_TID: u16 = 69;

pub enum TransferMode {
    NetAscii,
    Octet,
}

impl TransferMode {
    pub fn from_string(s: String) -> Option<TransferMode> {
        match s.as_ref() {
            "netascii" => Some(TransferMode::NetAscii),
            "octet" => Some(TransferMode::Octet),
            _ => None
        }
    }

    pub fn as_string(&self) -> String {
        match &self {
            NetAscii => String::from("netascii"),
            Octet => String::from("octet")
        }
    }

}

pub enum Opcode {
    Rrq = 0x01,
    Wrq = 0x02,
    Data = 0x03,
    Ack = 0x04,
    Error = 0x05
}

pub enum ErrorCode {
    Undefined = 0x00,
    FileNotFound = 0x01,
    AccessViolation = 0x02,
    DiskFull = 0x03,
    IllegalOperation = 0x04,
    UnknownTransferId = 0x05,
    FileExists = 0x06,
    NoSuchUser = 0x07
}
