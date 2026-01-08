//! Iced application adapter (thin UI layer)

use iced::time::{self, Duration};
use iced::{Element, Size, Subscription, Task};
use iced::window;
use std::cell::RefCell;
use tracing::info;

use crate::model::{App, Message, PlaybackState};
use crate::update;
use crate::view;

thread_local! {
    static INITIAL_TEXT: RefCell<Option<String>> = const { RefCell::new(None) };
}

pub fn set_initial_text(text: Option<String>) {
    INITIAL_TEXT.with(|t| *t.borrow_mut() = text);
}

pub fn new() -> (App, Task<Message>) {
    let text = INITIAL_TEXT.with(|t| t.borrow_mut().take());
    let app = App::new(text);
    
    // Log initial state for debugging
    if let Some(ref pending) = app.pending_text {
        info!(bytes = pending.len(), "App created with pending text");
    } else {
        info!("App created with no pending text");
    }
    
    // Open the main window (daemon doesn't open one by default)
    let (_main_window_id, open_task) = window::open(window::Settings {
        size: Size::new(360.0, 70.0),
        resizable: false,
        decorations: false,
        transparent: true,
        visible: true,
        ..Default::default()
    });
    let open_task = open_task.map(Message::WindowOpened);
    
    // Send a delayed message to initialize TTS if WindowOpened event didn't fire
    // This is a fallback for cases where the initial window doesn't trigger the event
    let init_task = if app.pending_text.is_some() {
        Task::perform(
            async {
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            },
            |_| Message::InitIfReady,
        )
    } else {
        Task::none()
    };
    
    (app, Task::batch([open_task, init_task]))
}

pub fn title(app: &App, window: window::Id) -> String {
    // Set different titles for different windows
    if app.settings_window_id == Some(window) {
        String::from("Settings")
    } else {
        String::from("Speaking...")
    }
}

pub fn update(app: &mut App, message: Message) -> Task<Message> {
    update::update(app, message)
}

pub fn view(app: &App, window: window::Id) -> Element<'_, Message> {
    // Show settings window if this is the settings window
    if app.settings_window_id == Some(window) {
        return view::settings_window_view(app);
    }
    
    view::main_view(app)
}

pub fn subscription(app: &App) -> Subscription<Message> {
    // Subscribe to window open/close events
    let window_opened = window::open_events().map(|id| {
        Message::WindowOpened(id)
    });
    
    let window_closed = window::close_events().map(|id| {
        Message::WindowClosed(id)
    });
    
    // Run animation/polling at ~75ms intervals
    // Only poll when playing (not stopped)
    let tick = match app.playback_state {
        PlaybackState::Stopped => Subscription::none(),
        _ => time::every(Duration::from_millis(75)).map(|_| Message::Tick),
    };
    
    Subscription::batch(vec![window_opened, window_closed, tick])
}
