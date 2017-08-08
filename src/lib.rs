// https://tools.ietf.org/html/rfc1350

const BLOCK_LENGTH      : u16   = 512;
const REQUESTING_TID    : u8    = 69;

enum TransferMode {
    NetAscii    = "netascii"    : &'static str,
    Octet       = "octet"       : &'static str,
    Mail        = "mail"        : &'static str
}

enum OpCode {
    Rrq     = 1 : u16,
    Wrq     = 2 : u16,
    Data    = 3 : u16,
    Ack     = 4 : u16,
    Error   = 5 : u16
}

enum ErrorCode {
    Undefined           = 0 : u16,
    FileNotFound        = 1 : u16,
    AccessViolation     = 2 : u16,
    DiskFull            = 3 : u16,
    IllegalOperation    = 4 : u16,
    UnknownTransferId   = 5 : u16,
    FileExists          = 6 : u16,
    NoSuchUser          = 7 : u16
}

struct RrqPacket {
    OpCode: OpCode,
    Filename: &str,
    Mode: TransferMode
}

struct WrqPacket {
    OpCode: OpCode,
    Filename: &str,
    Mode: TransferMode
}

struct DataPacket {
    OpCode: OpCode,
    BlockNum: u16,
    Data: Vec<u8>
}

struct AckPacket {
    OpCode: OpCode,
    BlockNum: u16,
}

struct ErrorPacket {
    OpCode: OpCode,
    ErrorCode: ErrorCode,
    ErrorMessage: &str
}
