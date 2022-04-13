use ciborium::de::from_reader;
use ciborium::ser::into_writer;
use log::{info, trace};
use mandos_lib::{ClientMessage, ServerMessage};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
    env_logger::init();

    info!("Creating listeners");
    let ipv4listener = TcpListener::bind("127.0.0.1:25566")
        .expect("Failed to create Listener, is another server already running?");
    let ipv6listener = TcpListener::bind("::1:25565")
        .expect("Failed to create Listener, is another server already running?");

    listen(ipv4listener);
    listen(ipv6listener);
}

fn listen(listener: TcpListener) {
    thread::spawn(move || {
        for stream in listener.incoming() {
            let stream = stream.unwrap();

            info!("Spawning thread for new connection");
            thread::spawn(|| server_handle(stream));
        }
    });
}

fn server_handle(stream: TcpStream) {
    loop {
        match from_reader(&stream) {
            Ok(message) => {
                info!("Received Message: {:?}", message);
                match message {
                    ClientMessage::RequestMoisture => {
                        into_writer(&ServerMessage::Moisture(0.5), &stream).unwrap();
                    }
                    ClientMessage::RequestWatering(_) => {
                        into_writer(&ServerMessage::WateringSuccess, &stream).unwrap();
                    }
                }
            }
            _ => {
                trace!("Connection closed - Shutting down thread");
                break;
            }
        }
        trace!("looping...");
    }
}
