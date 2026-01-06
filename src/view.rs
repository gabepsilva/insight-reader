/// UI rendering logic

use std::f32::consts::TAU;

use iced::widget::{button, column, container, row, text, progress_bar, svg, Space};
use iced::theme::Button;
use iced::{Alignment, Color, Element, Length};

use crate::model::{App, Message, PlaybackState};
use crate::styles::{CircleButtonStyle, WaveBarStyle, WindowStyle};

const MIN_HEIGHT: f32 = 8.0;
const MAX_HEIGHT: f32 = 24.0;
const NUM_BARS: usize = 10;

/// Calculate animated wave bar height using sine wave.
/// Each bar has a phase offset of 10 units (i.e., bars complete one full cycle over 10 bars).
fn animated_height(bar_index: usize, wave_offset: u32) -> f32 {
    let phase = (wave_offset as f32 + bar_index as f32 * 10.0) / 100.0 * TAU;
    let wave_factor = (phase.sin() + 1.0) / 2.0; // Normalize to 0-1
    MIN_HEIGHT + wave_factor * (MAX_HEIGHT - MIN_HEIGHT)
}

/// Helper to create a 36x36 circle button with centered content
fn circle_button<'a>(content: impl Into<Element<'a, Message>>, msg: Message) -> Element<'a, Message> {
    button(
        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
    )
    .width(Length::Fixed(36.0))
    .height(Length::Fixed(36.0))
    .style(Button::Custom(Box::new(CircleButtonStyle)))
    .on_press(msg)
    .into()
}

/// Helper to create an SVG icon element
fn icon(path: &str, size: f32) -> svg::Svg {
    svg(svg::Handle::from_path(path))
        .width(Length::Fixed(size))
        .height(Length::Fixed(size))
}

pub fn view(app: &App) -> Element<'_, Message> {
    // Waveform visualization with animated heights
    let waveform: Element<Message> = row(
        (0..NUM_BARS).map(|i| {
            let height = animated_height(i, app.wave_offset);
            container(Space::new(Length::Fixed(3.0), Length::Fixed(height)))
                .style(iced::theme::Container::Custom(Box::new(WaveBarStyle)))
                .into()
        }).collect::<Vec<Element<Message>>>()
    )
    .spacing(4)
    .align_items(Alignment::Center)
    .into();

    // Play/pause icon path based on state
    let play_pause_icon = if app.playback_state == PlaybackState::Playing {
        "assets/icons/pause.svg"
    } else {
        "assets/icons/play.svg"
    };

    let controls = row![
        circle_button(text("-5s").size(12).style(Color::WHITE), Message::SkipBackward),
        circle_button(text("+5s").size(12).style(Color::WHITE), Message::SkipForward),
        circle_button(icon(play_pause_icon, 16.0), Message::PlayPause),
        circle_button(icon("assets/icons/stop.svg", 16.0), Message::Stop),
    ]
    .spacing(6)
    .align_items(Alignment::Center);

    let main_bar = row![
        icon("assets/icons/volume.svg", 24.0),
        Space::with_width(Length::Fixed(16.0)),
        waveform,
        Space::with_width(Length::Fixed(16.0)),
        controls,
        Space::with_width(Length::Fixed(12.0)),
        icon("assets/icons/settings.svg", 16.0),
    ]
    .padding([8, 20, 4, 20])
    .align_items(Alignment::Center);

    let progress_bar = container(
        progress_bar(0.0..=1.0, app.progress).height(Length::Fixed(2.0))
    )
    .padding([0, 27, 8, 49]);

    container(column![main_bar, progress_bar])
        .width(Length::Fill)
        .height(Length::Fill)
        .style(iced::theme::Container::Custom(Box::new(WindowStyle)))
        .into()
}

