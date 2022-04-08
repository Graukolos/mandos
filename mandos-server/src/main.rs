use ciborium::de::from_reader;
use ciborium::ser::into_writer;
use log::{info, trace};
use mandos_lib::{ClientMessage, ServerMessage};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
    env_logger::init();

    info!("Creating listener");
    let listener = TcpListener::bind("localhost:25566")
        .expect("Failed to create Listener, is another server already running?");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        info!("Spawning thread for new connection");
        thread::spawn(|| server_handle(stream));
    }
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
