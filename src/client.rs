use std::net::UdpSocket;

mod client {
    pub fn read(host: &str, filename: &str) -> Result<File> {}
    pub fn write(host: &str, filename: &str) -> Result<()> {}
}

struct Client {
    socket: UdpSocket,
    filename: &str
}

impl Client {
    fn new(host: &str, filename: &str) -> Client {
        let mut socket;

        match UdpSocket::bind((host, REQUESTING_TID)) {
            Ok(s) => socket = s,
            e @ Error => e
        };

        Client {
            socket,
            filename
        }
    }
}
