/// Domain model for the application state

#[derive(Debug, Clone, PartialEq)]
pub enum PlaybackState {
    Stopped,
    Playing,
    Paused,
}

#[derive(Debug, Clone)]
pub enum Message {
    SkipBackward,
    SkipForward,
    PlayPause,
    Stop,
    Tick,
}

#[derive(Debug, Clone)]
pub struct App {
    pub playback_state: PlaybackState,
    pub progress: f32,
    pub wave_offset: u32,
}

impl App {
    pub fn new() -> Self {
        Self {
            playback_state: PlaybackState::Playing,
            progress: 0.35,
            wave_offset: 0,
        }
    }
}

