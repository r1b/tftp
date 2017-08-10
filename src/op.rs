use rfc1350::Opcode;

trait Operation<T: Opcode> {
    fn serialize(&self) -> [u8];
    fn deserialize([u8]) -> Operation<T>
}

struct Rrq {
    Opcode: Opcode,
    Filename: &str,
    Mode: TransferMode
}

impl Operation<Opcode::Rrq> for Rrq {
    fn serialize(&self) -> [u8] {}
    fn deserialize([u8]) -> Operation<Rrq> {}
}

struct Wrq {
    Opcode: Opcode,
    Filename: &str,
    Mode: TransferMode
}

impl Operation<Opcode::Wrq> for Wrq {
    fn serialize(&self) -> [u8] {}
    fn deserialize([u8]) -> Operation<Rrq> {}
}

struct Data {
    Opcode: Opcode,
    BlockNum: u16,
    Data: [u8]
}

impl Operation<Opcode::Data> for Data {
    fn serialize(&self) -> [u8] {}
    fn deserialize([u8]) -> Operation<Rrq> {}
}

struct Ack {
    Opcode: Opcode,
    BlockNum: u16,
}

impl Operation<Opcode::Ack> for Ack {
    fn serialize(&self) -> [u8] {}
    fn deserialize([u8]) -> Operation<Rrq> {}
}

struct Error {
    Opcode: Opcode,
    ErrorCode: ErrorCode,
    ErrorMessage: &str
}

impl Operation<Opcode::Error> for Error {
    fn serialize(&self) -> [u8] {}
    fn deserialize([u8]) -> Operation<Rrq> {}
}
