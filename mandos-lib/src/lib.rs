use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ClientMessage {
    RequestWatering(u8),
    RequestMoisture,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ServerMessage {
    Moisture(f32),
    WateringSuccess,
}
