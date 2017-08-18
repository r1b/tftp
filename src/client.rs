use std::fs::File;
use std::net::UdpSocket;
use std::io::Result;
use rfc1350::REQUESTING_TID;
/*
pub fn read(host: &str, filename: &str) -> Result<File> {}
pub fn write(host: &str, filename: &str) -> Result<()> {}

struct Client {
    socket: UdpSocket,
    filename: &'static str
}

impl Client {
    fn new(host: &str, filename: &str) -> Result<Client> {
        let mut socket = UdpSocket::bind((host, REQUESTING_TID))?;
        Client { socket, filename }
    }
}
*/
