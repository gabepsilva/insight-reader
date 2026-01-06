/// Business logic for state transitions

use crate::model::{App, Message, PlaybackState};

pub fn update(app: &mut App, message: Message) {
    match message {
        Message::SkipBackward => {
            app.progress = (app.progress - 0.1).max(0.0);
        }
        Message::SkipForward => {
            app.progress = (app.progress + 0.1).min(1.0);
        }
        Message::PlayPause => {
            app.playback_state = match app.playback_state {
                PlaybackState::Playing => PlaybackState::Paused,
                _ => PlaybackState::Playing,
            };
        }
        Message::Stop => {
            app.playback_state = PlaybackState::Stopped;
            app.progress = 0.0;
        }
        Message::Tick => {
            // Advance wave animation offset (wraps at 100)
            app.wave_offset = (app.wave_offset + 1) % 100;
        }
    }
}

