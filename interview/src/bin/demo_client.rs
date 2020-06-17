use interview::client::*;

fn main() {
    let client = Client::new("127.0.0.1:5000").unwrap();
    let buf = "Hello world".as_bytes();
    client.send_to_server(buf, "127.0.0.1:4000");
}