/// Entry point and window configuration

mod app;
mod model;
mod providers;
mod styles;
mod system;
mod update;
mod view;

use iced::{Application, Settings, Size};

use crate::model::App;

fn main() -> iced::Result {
    // Read selected text at startup
    if let Some(text) = crate::system::get_selected_text() {
        eprintln!("Text Selected: {} bytes", text.len());
    } else {
        eprintln!("No text selected");
    }

    App::run(Settings {
        window: iced::window::Settings {
            size: Size::new(380.0, 70.0),
            resizable: false,
            decorations: false,
            transparent: true,
            ..Default::default()
        },
        ..Default::default()
    })
}
