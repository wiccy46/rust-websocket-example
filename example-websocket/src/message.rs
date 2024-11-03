use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "data")]
pub enum ClientMessage {
    Command(CommandData),
    Parameter(ParameterData),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandData {
    pub rec: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParameterData {
    pub amplitude: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "data")]
pub enum ServerMessage {
    Ack(AckData),
    Error(ErrorData),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AckData {
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorData {
    pub message: String,
}