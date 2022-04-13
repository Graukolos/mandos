use ciborium::de::from_reader;
use ciborium::ser::into_writer;
use dns_lookup::lookup_host;
use log::{debug, info, warn};
use mandos_lib::{ClientMessage, ServerMessage};
use std::net::{IpAddr, SocketAddr, TcpStream};
use std::str::FromStr;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

pub fn connect(server_address: &str) -> std::io::Result<TcpStream> {
    info!("Trying to connect to {}", server_address);
    let ip_addrs = match IpAddr::from_str(format!("{}:25566", server_address).as_str()) {
        Ok(v) => vec![v],
        Err(_) => match lookup_host(server_address) {
            Ok(v) => v,
            Err(e) => return Err(e),
        },
    };

    let mut result = None;

    for ip_addr in ip_addrs.iter() {
        debug!("{}", ip_addr);
        match TcpStream::connect_timeout(
            &SocketAddr::new(*ip_addr, 25566),
            Duration::from_millis(500),
        ) {
            Ok(stream) => return Ok(stream),
            Err(e) => result = Some(Err(e)),
        };
    }

    result.unwrap()
}

fn send(stream: &TcpStream, message: &ClientMessage) {
    into_writer(message, stream).unwrap();
}

fn receive(stream: &TcpStream) -> ServerMessage {
    from_reader(stream).unwrap()
}

pub fn worker(stream: TcpStream, rx: Receiver<u8>, tx: Sender<f32>) -> JoinHandle<()> {
    thread::spawn(move || loop {
        info!("loop");
        thread::sleep(Duration::from_millis(500));
        if let Ok(secs) = rx.try_recv() {
            info!("sending watering request");
            send(&stream, &ClientMessage::RequestWatering(secs));
            info!("receiving answer");
            if let ServerMessage::WateringSuccess = receive(&stream) {
                info!("success!");
            } else {
                warn!("no watering success received");
            }
        }
        send(&stream, &ClientMessage::RequestMoisture);
        if let ServerMessage::Moisture(v) = receive(&stream) {
            tx.send(v).unwrap();
        } else {
            warn!("moisture value not received")
        }
    })
}
