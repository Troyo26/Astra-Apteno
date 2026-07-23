use crate::app::{Message, Widget};
use crate::models::{Sortie, SortieVariant};
use crate::ui::components::{divider, header, style};
use crate::utils::time::remaining;
use iced::widget::{column, container, row, text};
use iced::{Element, Fill};

// Functions

fn short_modifier(modifier: &str) -> &str {
    match modifier {
        "Environmental Hazard: Radiation Pockets" => "Radiation Hazard",
        "Environmental Hazard: Magnetic Anomalies" => "Magnetic Hazard",
        "Environmental Hazard: Cryogenic Leak" => "Cold Hazard",
        "Enemy Elemental Enhancement: Radiation" => "Radiation Enemies",
        "Enemy Elemental Enhancement: Corrosive" => "Corrosive Enemies",
        "Enemy Elemental Enhancement: Toxin" => "Toxin Enemies",
        "Enemy Physical Enhancement" => "Physical Enemies",
        "Eximus Stronghold" => "Eximus",
        "Weapon Restriction: Shotgun Only" => "Shotguns Only",
        "Weapon Restriction: Bow Only" => "Bows Only",
        "Weapon Restriction: Sniper Only" => "Snipers Only",
        "Weapon Restriction: Pistol Only" => "Pistol Only",
        other => other,
    }
}

fn mission_view<'a>(variant: &'a SortieVariant) -> Element<'a, Message> {
    container(row![
        container(text(&variant.mission_type))
            .width(Fill)
            .align_left(Fill),
        container(text(&variant.node)).width(Fill).center_x(Fill),
        container(text(short_modifier(&variant.modifier)))
            .width(Fill)
            .align_right(Fill),
    ])
    .padding(8)
    .into()
}

fn status(_sortie: &Sortie) -> &'static str {
    "Active"
}

// Compact Widget

fn compact(sortie: &Sortie) -> Element<'_, Message> {
    container(header::view(
        "Sortie",
        Some(remaining(&sortie.expiry)),
        false,
        Message::ToggleWidget(Widget::Sortie),
    ))
    .width(500)
    .style(style::widget)
    .into()
}

// Expanded Widget

fn expanded_widget(sortie: &Sortie) -> Element<'_, Message> {
    let mut content = column![
        header::view(
            "Sortie",
            Some(remaining(&sortie.expiry)),
            true,
            Message::ToggleWidget(Widget::Sortie),
        ),
        divider::view(),
        container(row![
            container(text(format!("{}", status(sortie))))
                .width(Fill)
                .align_left(Fill),
            container(text(format!("{}", sortie.boss)))
                .width(Fill)
                .center_x(Fill),
            container(text(format!("{}", sortie.faction)))
                .width(Fill)
                .align_right(Fill),
        ])
        .padding(8),
    ];

    for variant in &sortie.variants {
        content = content.push(divider::view());
        content = content.push(mission_view(variant));
    }

    container(content).width(500).style(style::widget).into()
}

// View Function
pub fn view(sortie: &Sortie, expanded: bool) -> Element<'_, Message> {
    if expanded {
        expanded_widget(sortie)
    } else {
        compact(sortie)
    }
}
