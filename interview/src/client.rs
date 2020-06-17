use std::net::UdpSocket;
use std::io;
use std::str::from_utf8;

pub struct Client {
    socket: UdpSocket,
}

impl Client {
    pub fn new(socket: &str) -> Result<Client, io::Error> {
        let udp_socket = match UdpSocket::bind(socket) {
            Ok(u) => u,
            Err(e) => return Err(e),
        };

        let client = Client {
            socket: udp_socket,
        };

        Ok(client)
    }

    pub fn send_to_server(&self, buf: &[u8], server_addr: &str) {
        self.socket.send_to(buf, server_addr);
        let mut message_buff = [0; 100];
        let (number_bytes, src_addr) = self.socket.recv_from(&mut message_buff).expect("Didn't recieve data!");
        let filled_buff = &mut message_buff[..number_bytes];
        let message = from_utf8(filled_buff).unwrap();
        print!("{}", message);
    }
}