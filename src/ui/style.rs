use iced::widget::container;
use iced::{Background, Border, Color};

pub fn widget(_theme: &iced::Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::from_rgb8(32, 35, 42))),
        border: Border {
            radius: 12.0.into(),
            width: 1.0,
            color: Color::from_rgb8(70, 70, 70),
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
