use interview::server::*;

fn main() {
    let mut server = Server::new("127.0.0.1:4000").unwrap();
    let mut buf = [0; 10];
    server.listen(&mut buf);
}