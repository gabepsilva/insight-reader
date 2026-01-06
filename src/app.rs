/// Iced application adapter (thin UI layer)

use iced::time::{self, Duration};
use iced::{Application, Command, Element, Subscription, Theme};

use crate::model::{App, Message, PlaybackState};
use crate::update;
use crate::view;

impl Application for App {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (Self::new(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Speaking...")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        update::update(self, message);
        Command::none()
    }

    fn view(&self) -> Element<'_, Message> {
        view::view(self)
    }

    fn subscription(&self) -> Subscription<Message> {
        // Run animation at ~75ms intervals (matching dad project)
        // Only animate when not stopped
        match self.playback_state {
            PlaybackState::Stopped => Subscription::none(),
            _ => time::every(Duration::from_millis(75)).map(|_| Message::Tick),
        }
    }
}

