use std::net::UdpSocket;
use std::net::SocketAddr;
use std::io;
use std::str::from_utf8;

pub struct Server {
    socket: UdpSocket,
    clients: Vec<SocketAddr>,
}

impl Server {
    pub fn new(socket: &str) -> Result<Server, io::Error> {
        let udp_socket = match UdpSocket::bind(socket) {
            Ok(u) => u,
            Err(e) => return Err(e),
        };

        let server = Server {
            socket: udp_socket,
            clients: vec![],
        };

        Ok(server)
    }

    pub fn listen(&mut self, buf: &mut [u8]) {
        let (num_bytes, src_addr) = self.socket.recv_from(buf).expect("Didn't recieve data!");
        let send_buf;
        if self.clients.contains(&src_addr) {
            send_buf = format!("Welcome! Server has read your message: {}", from_utf8().unwrap());
        } else {
            self.clients.push(src_addr);
            send_buf = format!("Server has read your message: {}", buf);
        }
        self.socket.send_to(&send_buf[..].as_bytes(), src_addr);
    }
}