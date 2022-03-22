use std::net::TcpStream;
use ciborium::de::from_reader;
use mandos_lib::Message;

fn main() {
    let stream = TcpStream::connect("127.0.0.1:25565").unwrap();

    let message: Message = from_reader(stream).unwrap();

    println!("{:?}", message);
}
