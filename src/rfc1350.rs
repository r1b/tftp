// https://tools.ietf.org/html/rfc1350

const BLOCK_LENGTH      : u16   = 512;
const REQUESTING_TID    : u8    = 69;

enum TransferMode {
    NetAscii    = "netascii"    : &str,
    Octet       = "octet"       : &str,
    Mail        = "mail"        : &str
}

enum Opcode {
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
