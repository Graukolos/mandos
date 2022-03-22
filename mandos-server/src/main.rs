use std::net::{TcpListener, TcpStream};
use ciborium::ser::into_writer;
use mandos_lib::Message;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:25565").expect("Failed to create Listener, is another server already running?");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        server_handle(stream);
    }
}

fn server_handle(stream: TcpStream) {
    into_writer(&Message::Ciao, stream).unwrap();
}
