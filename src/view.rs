//! UI rendering logic

use iced::widget::{button, checkbox, column, container, mouse_area, progress_bar, radio, row, scrollable, svg, text, Space};
use iced::{Alignment, Background, Color, Element, Length};

use crate::model::{App, LogLevel, Message, PlaybackState, TTSBackend};
use crate::styles::{
    circle_button_style, close_button_style, error_container_style, header_style,
    modal_content_style, section_style, transparent_button_style, wave_bar_style,
    white_checkbox_style, white_radio_style, window_style,
};

const MIN_HEIGHT: f32 = 4.0;
const MAX_HEIGHT: f32 = 24.0;
const NUM_BARS: usize = 10;

// Bundled SVG icons (embedded at compile time)
const SVG_PLAY: &[u8] = include_bytes!("../assets/icons/play.svg");
const SVG_PAUSE: &[u8] = include_bytes!("../assets/icons/pause.svg");
const SVG_STOP: &[u8] = include_bytes!("../assets/icons/stop.svg");
const SVG_VOLUME: &[u8] = include_bytes!("../assets/icons/volume.svg");
const SVG_SETTINGS: &[u8] = include_bytes!("../assets/icons/settings.svg");

/// Calculate bar height from frequency band amplitude (0.0-1.0).
fn bar_height(amplitude: f32) -> f32 {
    MIN_HEIGHT + amplitude * (MAX_HEIGHT - MIN_HEIGHT)
}

/// Helper to create a 36x36 circle button with centered content.
fn circle_button<'a>(
    content: impl Into<Element<'a, Message>>,
    msg: Message,
) -> Element<'a, Message> {
    button(
        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill),
    )
    .width(Length::Fixed(36.0))
    .height(Length::Fixed(36.0))
    .style(circle_button_style)
    .on_press(msg)
    .into()
}

/// Helper to create an SVG icon element from bundled bytes.
fn icon_from_bytes(bytes: &'static [u8], size: f32) -> svg::Svg<'static> {
    svg(svg::Handle::from_memory(bytes))
        .width(Length::Fixed(size))
        .height(Length::Fixed(size))
}

/// Icon helper functions for each bundled SVG.
fn play_icon(size: f32) -> svg::Svg<'static> {
    icon_from_bytes(SVG_PLAY, size)
}

fn pause_icon(size: f32) -> svg::Svg<'static> {
    icon_from_bytes(SVG_PAUSE, size)
}

fn stop_icon(size: f32) -> svg::Svg<'static> {
    icon_from_bytes(SVG_STOP, size)
}

fn volume_icon(size: f32) -> svg::Svg<'static> {
    icon_from_bytes(SVG_VOLUME, size)
}

fn settings_icon(size: f32) -> svg::Svg<'static> {
    icon_from_bytes(SVG_SETTINGS, size)
}

/// Helper to create white text with consistent styling.
fn white_text(content: &str, size: u32) -> text::Text<'_> {
    text(content)
        .size(size)
        .style(|_theme| iced::widget::text::Style {
            color: Some(Color::WHITE),
        })
}

/// Helper to create red error text with consistent styling.
fn error_text(content: &str, size: u32) -> text::Text<'_> {
    text(content)
        .size(size)
        .style(|_theme| iced::widget::text::Style {
            color: Some(Color::from_rgb(1.0, 0.3, 0.3)),
        })
}

/// Settings window view - floating modal style
pub fn settings_window_view<'a>(app: &'a App) -> Element<'a, Message> {
    let close_button = button(
        container(white_text("✕", 18))
            .width(Length::Fixed(28.0))
            .height(Length::Fixed(28.0))
            .center_x(Length::Fixed(28.0))
            .center_y(Length::Fixed(28.0)),
    )
    .style(close_button_style)
    .on_press(Message::CloseSettings);

    // Error message display (if present)
    let error_display: Element<'a, Message> = if let Some(error_msg) = &app.error_message {
        container(
            error_text(error_msg, 13)
                .width(Length::Fill)
        )
        .width(Length::Fill)
        .padding(12)
        .style(error_container_style)
        .into()
    } else {
        column![].spacing(0).into()
    };

    // TTS Provider section
    let provider_controls = column![
        radio(
            "Piper (local, offline)",
            TTSBackend::Piper,
            Some(app.selected_backend),
            Message::ProviderSelected
        )
        .style(white_radio_style),
        Space::new().height(Length::Fixed(6.0)),
        radio(
            "AWS Polly (cloud)",
            TTSBackend::AwsPolly,
            Some(app.selected_backend),
            Message::ProviderSelected
        )
        .style(white_radio_style),
    ]
    .spacing(0);

    let provider_section = container(
        column![
            row![
                container(
                    white_text("TTS Provider", 14)
                        .style(|_theme| iced::widget::text::Style {
                            color: Some(Color::from_rgba(1.0, 1.0, 1.0, 0.85)),
                        })
                )
                .width(Length::Fixed(120.0))
                .align_x(Alignment::Start),
                Space::new().width(Length::Fixed(16.0)),
                container(provider_controls)
                    .width(Length::Fill)
                    .align_x(Alignment::Start),
            ]
            .align_y(Alignment::Center)
            .width(Length::Fill)
            .padding([12.0, 16.0]),
            error_display,
        ]
        .spacing(8)
    )
    .style(section_style);

    // Log Level section - compact horizontal layout
    let log_level_controls = row![
        radio("Error", LogLevel::Error, Some(app.log_level), Message::LogLevelSelected)
            .style(white_radio_style),
        radio("Warn", LogLevel::Warn, Some(app.log_level), Message::LogLevelSelected)
            .style(white_radio_style),
        radio("Info", LogLevel::Info, Some(app.log_level), Message::LogLevelSelected)
            .style(white_radio_style),
        radio("Debug", LogLevel::Debug, Some(app.log_level), Message::LogLevelSelected)
            .style(white_radio_style),
        radio("Trace", LogLevel::Trace, Some(app.log_level), Message::LogLevelSelected)
            .style(white_radio_style),
    ]
    .spacing(16);

    let log_level_section = container(
        row![
            container(
                white_text("Log Level", 14)
                    .style(|_theme| iced::widget::text::Style {
                        color: Some(Color::from_rgba(1.0, 1.0, 1.0, 0.85)),
                    })
            )
            .width(Length::Fixed(120.0))
            .align_x(Alignment::Start),
            Space::new().width(Length::Fixed(16.0)),
            container(log_level_controls)
                .width(Length::Fill)
                .align_x(Alignment::Start),
        ]
        .align_y(Alignment::Center)
        .width(Length::Fill)
        .padding([12.0, 16.0])
    )
    .style(section_style);

    // Text Cleanup section
    let text_cleanup_control = checkbox(app.text_cleanup_enabled)
        .label("Enable text cleanup (sends text to local API before TTS)")
        .on_toggle(Message::TextCleanupToggled)
        .style(white_checkbox_style);

    let text_cleanup_section = container(
        row![
            container(
                white_text("Text Cleanup", 14)
                    .style(|_theme| iced::widget::text::Style {
                        color: Some(Color::from_rgba(1.0, 1.0, 1.0, 0.85)),
                    })
            )
            .width(Length::Fixed(120.0))
            .align_x(Alignment::Start),
            Space::new().width(Length::Fixed(16.0)),
            container(text_cleanup_control)
                .width(Length::Fill)
                .align_x(Alignment::Start),
        ]
        .align_y(Alignment::Center)
        .width(Length::Fill)
        .padding([12.0, 16.0])
    )
    .style(section_style);

    container(
        column![
            // Header bar at top (fixed, not scrollable)
            container(
                row![
                    white_text("Settings", 20)
                        .style(|_theme| iced::widget::text::Style {
                            color: Some(Color::WHITE),
                        }),
                    Space::new().width(Length::Fill),
                    close_button,
                ]
                .width(Length::Fill)
                .align_y(Alignment::Center)
            )
            .width(Length::Fill)
            .padding([20.0, 24.0])
            .style(header_style),
            // Scrollable content area
            scrollable(
                container(
                    column![
                        provider_section,
                        Space::new().height(Length::Fixed(12.0)),
                        log_level_section,
                        Space::new().height(Length::Fixed(12.0)),
                        text_cleanup_section,
                    ]
                    .padding([20.0, 24.0])
                    .spacing(0)
                    .align_x(Alignment::Start),
                )
                .width(Length::Fill)
                .style(|_theme| container::Style {
                    background: Some(Background::Color(Color::from_rgb(0.12, 0.12, 0.14))),
                    ..Default::default()
                }),
            )
            .width(Length::Fill)
            .height(Length::Fill),
        ]
        .spacing(0)
        .width(Length::Fill)
        .height(Length::Fill),
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x(Length::Fill)
    .center_y(Length::Fill)
    .style(modal_content_style)
    .into()
}

/// Main window view
///
/// Layout structure (window is 380×70):
/// ┌──────────────────────────────────────────────────────┐
/// │  [vol] ||||||||  [-5s] [+5s] [▶] [■]          [⚙]   │
/// │  ════════════════════════════════════════════════    │
/// └──────────────────────────────────────────────────────┘
pub fn main_view(app: &App) -> Element<'_, Message> {
    // 1. Waveform: 10 vertical bars
    let waveform: Element<Message> = row((0..NUM_BARS)
        .map(|i| {
            let amplitude = app.frequency_bands.get(i).copied().unwrap_or(0.0);
            let height = bar_height(amplitude);
            container(
                Space::new()
                    .width(Length::Fixed(3.0))
                    .height(Length::Fixed(height)),
            )
            .style(wave_bar_style)
            .into()
        })
        .collect::<Vec<Element<Message>>>())
    .spacing(4)
    .align_y(Alignment::Center)
    .into();

    // 2. Play/pause icon
    let play_pause_icon: Element<Message> = if app.playback_state == PlaybackState::Playing {
        pause_icon(16.0).into()
    } else {
        play_icon(16.0).into()
    };

    // 3. Control buttons row
    let controls = row![
        circle_button(white_text("-5s", 12), Message::SkipBackward),
        circle_button(white_text("+5s", 12), Message::SkipForward),
        circle_button(play_pause_icon, Message::PlayPause),
        circle_button(stop_icon(16.0), Message::Stop),
    ]
    .spacing(6)
    .align_y(Alignment::Center);

    // 4. Base content row (without gear): [volume] [waveform] [controls]
    let content_row = row![
        volume_icon(28.0),
        Space::new().width(Length::Fixed(12.0)),
        waveform,
        Space::new().width(Length::Fixed(12.0)),
        controls,
    ]
    .align_y(Alignment::Center)
    .padding([8.0, 16.0]);

    // 5. Progress bar OR status text directly under the content row (not under gear)
    let (progress_or_status, gap_height): (Element<Message>, f32) = if let Some(status) = &app.status_text {
        // Show status text during loading (pushed up above where progress bar would be)
        let elem = container(
            text(status)
                .size(11)
                .style(|_theme| iced::widget::text::Style {
                    color: Some(Color::from_rgba(1.0, 1.0, 1.0, 0.7)),
                }),
        )
        .width(Length::Fixed(313.0))
        .height(Length::Fixed(33.0))
        .padding([-6.0, 19.0])
        .into();
        (elem, -8.0)
    } else {
        // Show progress bar during playback (stays in same position)
        let elem = container(progress_bar(0.0..=1.0, app.progress))
            .width(Length::Fixed(313.0))
            .height(Length::Fixed(1.0))
            .padding([0.0, 19.0])
            .into();
        (elem, 3.0)
    };

    let content_column = column![
        content_row,
        Space::new().height(Length::Fixed(gap_height)),
        progress_or_status,
    ]
    .width(Length::Shrink);

    // 6. Settings gear (transparent button) on the right
    let settings_btn = button(settings_icon(18.0))
        .style(transparent_button_style)
        .padding([0.0, 0.0])
        .on_press(Message::Settings);

    // 7. Final row: [content_column | spacer | gear], centered with padding
    let content = row![
        content_column,
        Space::new().width(Length::Fill),
        settings_btn,
    ]
    .align_y(Alignment::Center)
    .padding([4.0, 10.0]); // [top/bottom, left/right]

    // 8. Outer container with window styling, wrapped in mouse_area for dragging
    mouse_area(
        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(window_style),
    )
    .on_press(Message::StartDrag)
    .into()
}
