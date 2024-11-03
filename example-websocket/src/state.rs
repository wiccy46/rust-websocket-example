#[derive(Debug)]
pub struct AudioState {
    pub rec: bool,
    pub amplitude: f32,
}

impl AudioState {
    pub fn new() -> Self {
        Self { rec: false, amplitude: 1.0 }
    }
}