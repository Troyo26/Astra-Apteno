use crate::app::{Message, Widget};
use crate::models::{Activity, AlertsEvents};
use crate::ui::components::{divider, header, style};
use crate::utils::time::remaining;
use iced::widget::{column, container, row, text};
use iced::{Element, Fill};
use std::collections::HashSet;

// Functions

fn activity_details<'a>(activity: &'a Activity) -> Element<'a, Message> {
    let mut content = column![
        row![
            container(text(format!("Node"))).align_left(Fill),
            container(text(format!("{}", activity.node))).align_right(Fill),
        ],
        row![
            container(text(format!("Started"))).align_left(Fill),
            container(text(format!("{}", activity.activation))).align_right(Fill),
        ],
        row![
            container(text(format!("Ends"))).align_left(Fill),
            container(text(format!("{}", activity.expiry))).align_right(Fill)
        ],
    ]
    .spacing(4);

    content = content.push(divider::view());

    if !activity.rewards.is_empty() {
        content = content.push(text("Rewards"));

        for reward in &activity.rewards {
            for item in &reward.items {
                content =
                    content.push(row![container(text(format!("{}", item))).align_right(Fill)]);
            }
        }
    }

    content = content.push(divider::view());

    if !activity.interim_steps.is_empty() {
        content = content.push(text("Milestone Rewards:"));

        for step in &activity.interim_steps {
            for item in &step.reward.items {
                content = content.push(row![
                    container(text(format!("{}", step.goal))).align_left(Fill),
                    container(text(format!("{}", item))).align_right(Fill),
                ]);
            }
        }
    }

    container(content).padding(8).into()
}

fn activity_header(activity: &Activity, expanded: bool) -> Element<'_, Message> {
    container(header::view(
        &activity.description,
        Some(remaining(&activity.expiry)),
        expanded,
        Message::ToggleActivity(activity.id.clone()),
    ))
    .width(500)
    .style(style::widget)
    .into()
}

fn activities_view<'a>(
    alerts_events: &'a AlertsEvents,
    expanded_activities: &HashSet<String>,
) -> Element<'a, Message> {
    let mut content = column![];

    if !alerts_events.events.is_empty() {
        content = content.push(
            container(text("Events"))
                .center_x(Fill)
                .style(style::widget),
        );

        for event in &alerts_events.events {
            let expanded = expanded_activities.contains(&event.id);
            content = content.push(activity_header(event, expanded));

            if expanded {
                content = content.push(activity_details(event));
            }
        }
    }

    if !alerts_events.alerts.is_empty() {
        content = content.push(divider::view());
        content = content.push(text("Alerts"));

        for alert in &alerts_events.alerts {
            let expanded = expanded_activities.contains(&alert.id);
            content = content.push(activity_header(alert, expanded));

            if expanded {
                content = content.push(activity_details(alert));
            }
        }
    }

    if alerts_events.events.is_empty() && alerts_events.alerts.is_empty() {
        return text("No active alerts and events.").into();
    }

    content.into()
}

/* fn status(arbitration: &Arbitration) -> &'static str {
    if arbitration.mission_type == "Unknown" {
        "Inactive"
    } else {
        "Active"
    }
} */

// Compact Widget

fn compact(_alerts_events: &AlertsEvents) -> Element<'_, Message> {
    container(header::view(
        "Alerts and Events",
        None,
        false,
        Message::ToggleWidget(Widget::AlertsEvents),
    ))
    .width(500)
    .style(style::widget)
    .into()
}

// Expanded Widget

fn expanded_widget<'a>(
    alertsevents: &'a AlertsEvents,
    expanded_activities: &HashSet<String>,
) -> Element<'a, Message> {
    let mut content = column![header::view(
        "Alerts and Events",
        None,
        true,
        Message::ToggleWidget(Widget::AlertsEvents),
    )];

    content = content.push(divider::view());
    content = content.push(activities_view(alertsevents, expanded_activities));

    container(content).width(500).style(style::widget).into()
}

// View Function

pub fn view<'a>(
    alertsevents: &'a AlertsEvents,
    expanded: bool,
    expanded_activities: &HashSet<String>,
) -> Element<'a, Message> {
    if expanded {
        expanded_widget(alertsevents, expanded_activities)
    } else {
        compact(alertsevents)
    }
}
