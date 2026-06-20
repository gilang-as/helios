use cosmic::Element;
use cosmic::iced::{Alignment, Color, Length, Padding};
use cosmic::widget::{container, mouse_area, text};

use crate::Message;

fn icon_color(icon: &str) -> Color {
    match icon {
        "folder" => Color::from_rgb(0.95, 0.75, 0.25),
        "terminal" => Color::from_rgb(0.2, 0.8, 0.4),
        "settings" => Color::from_rgb(0.6, 0.6, 0.7),
        "browser" => Color::from_rgb(0.25, 0.55, 0.95),
        "editor" => Color::from_rgb(0.85, 0.35, 0.35),
        _ => Color::from_rgb(0.5, 0.5, 0.5),
    }
}

fn icon_label(icon: &str) -> &str {
    match icon {
        "folder" => "F",
        "terminal" => ">_",
        "settings" => "S",
        "browser" => "B",
        "editor" => "E",
        _ => "?",
    }
}

pub fn app_item<'a>(
    index: usize,
    _name: &'a str,
    icon: &'a str,
    hovered: bool,
    size: u32,
) -> Element<'a, Message> {
    let color = icon_color(icon);
    let icon_px = size as f32;
    let scale = if hovered { 1.15_f32 } else { 1.0_f32 };
    let scaled = (icon_px * scale) as u32;

    let icon_widget = container(
        text(icon_label(icon))
            .size((scaled as f32 * 0.38).max(10.0))
            .class(Color::WHITE),
    )
    .width(scaled as f32)
    .height(scaled as f32)
    .padding(Padding::new(0.0))
    .style(
        move |_theme: &cosmic::Theme| cosmic::iced::widget::container::Style {
            background: Some(cosmic::iced::Background::Color(color)),
            border: cosmic::iced::Border {
                radius: (scaled as f32 * 0.22).into(),
                ..Default::default()
            },
            ..Default::default()
        },
    )
    .align_x(Alignment::Center)
    .align_y(Alignment::Center);

    // Wrap in fixed-size container so layout doesn't shift on hover
    let fixed = container(icon_widget)
        .width(Length::Fixed(icon_px + 10.0))
        .height(Length::Fixed(icon_px + 10.0))
        .align_x(Alignment::Center)
        .align_y(Alignment::Center);

    mouse_area(fixed)
        .on_enter(Message::AppHovered(index))
        .on_exit(Message::AppLeft)
        .on_press(Message::AppClicked(index))
        .into()
}
