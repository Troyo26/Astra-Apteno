use iced::widget::{button, container};
use iced::{Background, Border, Color};

//Widget v1
/* pub fn widget(_theme: &iced::Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::from_rgb8(32, 35, 42))),
        border: Border {
            radius: 12.0.into(),
            width: 1.0,
            color: Color::from_rgb8(70, 70, 70),
        },
        ..Default::default()
    }
} */

//Widget v2
pub fn widget(_theme: &iced::Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::from_rgb8(32, 35, 42))),
        border: iced::Border {
            radius: 10.into(),
            width: 1.0,
            color: Color::from_rgb8(55, 60, 70),
        },
        ..Default::default()
    }
}

pub fn divider(_theme: &iced::Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::from_rgb8(70, 70, 70))),
        border: Border::default(),
        ..Default::default()
    }
}

pub fn background(_theme: &iced::Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::from_rgb8(18, 20, 25))),
        ..Default::default()
    }
}

pub fn menu_button(_theme: &iced::Theme, status: button::Status) -> button::Style {
    match status {
        button::Status::Active => button::Style {
            background: Some(Background::Color(Color::from_rgb8(35, 38, 45))),
            text_color: Color::WHITE,
            border: Border {
                radius: 8.0.into(),
                width: 1.0,
                color: Color::from_rgb8(60, 65, 75),
            },
            ..Default::default()
        },

        button::Status::Hovered => button::Style {
            background: Some(Background::Color(Color::from_rgb8(50, 55, 65))),
            text_color: Color::WHITE,
            border: Border {
                radius: 8.0.into(),
                width: 1.0,
                color: Color::from_rgb8(100, 180, 255),
            },
            ..Default::default()
        },

        button::Status::Pressed => button::Style {
            background: Some(Background::Color(Color::from_rgb8(25, 120, 200))),
            text_color: Color::WHITE,
            border: Border {
                radius: 8.0.into(),
                width: 1.0,
                color: Color::from_rgb8(100, 180, 255),
            },
            ..Default::default()
        },

        _ => button::Style::default(),
    }
}
