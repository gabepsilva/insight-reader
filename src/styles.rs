//! Custom style functions for UI components (Iced 0.13+ closure-based API)

use iced::widget::{button, checkbox, container, radio};
use iced::{Background, Border, Color, Theme};

pub fn window_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::BLACK)),
        border: Border {
            color: Color::WHITE,
            width: 1.0,
            radius: 8.0.into(),
        },
        ..Default::default()
    }
}

pub fn wave_bar_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.6))),
        border: Border {
            radius: 1.5.into(),
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn circle_button_style(_theme: &Theme, status: button::Status) -> button::Style {
    let base_bg = match status {
        button::Status::Active => Color::from_rgba(1.0, 1.0, 1.0, 0.15),
        button::Status::Hovered => Color::from_rgba(1.0, 1.0, 1.0, 0.25),
        button::Status::Pressed => Color::from_rgba(1.0, 1.0, 1.0, 0.35),
        _ => Color::from_rgba(1.0, 1.0, 1.0, 0.15),
    };

    button::Style {
        background: Some(Background::Color(base_bg)),
        text_color: Color::WHITE,
        border: Border {
            radius: 18.0.into(),
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn modal_content_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::from_rgb(0.12, 0.12, 0.14))),
        border: Border {
            color: Color::from_rgba(1.0, 1.0, 1.0, 0.15),
            width: 1.0,
            radius: 12.0.into(),
        },
        ..Default::default()
    }
}

/// Style for section containers (grouped settings)
pub fn section_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::from_rgb(0.08, 0.08, 0.10))),
        border: Border {
            color: Color::from_rgba(1.0, 1.0, 1.0, 0.08),
            width: 1.0,
            radius: 8.0.into(),
        },
        ..Default::default()
    }
}

/// Style for the close button in settings
pub fn close_button_style(_theme: &Theme, status: button::Status) -> button::Style {
    let base_bg = match status {
        button::Status::Active => Color::TRANSPARENT,
        button::Status::Hovered => Color::from_rgba(1.0, 1.0, 1.0, 0.15),
        button::Status::Pressed => Color::from_rgba(1.0, 1.0, 1.0, 0.25),
        _ => Color::TRANSPARENT,
    };

    button::Style {
        background: Some(Background::Color(base_bg)),
        text_color: Color::from_rgba(1.0, 1.0, 1.0, 0.7),
        border: Border {
            radius: 6.0.into(),
            ..Default::default()
        },
        ..Default::default()
    }
}

/// Style for error message containers
pub fn error_container_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::from_rgba(1.0, 0.2, 0.2, 0.15))),
        border: Border {
            color: Color::from_rgb(1.0, 0.3, 0.3),
            width: 1.0,
            radius: 6.0.into(),
        },
        ..Default::default()
    }
}

/// Style for the header bar at the top of settings
pub fn header_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::from_rgb(0.10, 0.10, 0.12))),
        border: Border {
            color: Color::from_rgba(1.0, 1.0, 1.0, 0.1),
            width: 0.0,
            radius: 0.0.into(),
        },
        ..Default::default()
    }
}

/// Transparent button style for icon-only controls (e.g., settings gear).
pub fn transparent_button_style(_theme: &Theme, _status: button::Status) -> button::Style {
    button::Style {
        background: None,
        text_color: Color::WHITE,
        border: Border::default(),
        ..Default::default()
    }
}

/// White text radio style for dark backgrounds.
pub fn white_radio_style(_theme: &Theme, _status: radio::Status) -> radio::Style {
    radio::Style {
        background: Background::Color(Color::TRANSPARENT),
        dot_color: Color::from_rgb(0.4, 0.6, 1.0),
        border_width: 1.0,
        border_color: Color::from_rgba(1.0, 1.0, 1.0, 0.6),
        text_color: Some(Color::WHITE),
    }
}

/// White text checkbox style for dark backgrounds.
pub fn white_checkbox_style(_theme: &Theme, status: checkbox::Status) -> checkbox::Style {
    let is_checked = match status {
        checkbox::Status::Active { is_checked } => is_checked,
        checkbox::Status::Hovered { is_checked } => is_checked,
        checkbox::Status::Disabled { is_checked } => is_checked,
    };
    checkbox::Style {
        background: Background::Color(if is_checked {
            Color::from_rgb(0.4, 0.6, 1.0)
        } else {
            Color::TRANSPARENT
        }),
        icon_color: Color::WHITE,
        border: Border {
            color: Color::from_rgba(1.0, 1.0, 1.0, 0.6),
            width: 1.0,
            radius: 3.0.into(),
        },
        text_color: Some(Color::WHITE),
    }
}
