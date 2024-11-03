use crate::message::{CommandData, ParameterData};
use crate::state::AudioState;
use tokio::sync::Mutex;
use std::sync::Arc;

pub async fn handle_command(cmd: CommandData, audio_state: Arc<Mutex<AudioState>>) {
    let mut audio_state = audio_state.lock().await;
    audio_state.rec = cmd.rec;
    if cmd.rec {
        println!("Start recording");
    } else {
        println!("Stop recording");
    }

}

pub async fn handle_parameter(param: ParameterData, audio_state: Arc<Mutex<AudioState>>) {
    let mut audio_state = audio_state.lock().await;
    // Can add data validation here
    if let Some(amplitude) = param.amplitude {
        audio_state.amplitude = amplitude;
        println!("Set amplitude to {}", amplitude);
    }
}
