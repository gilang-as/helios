use iced::{
    Alignment, Color, Element, Length,
    widget::{column, container, mouse_area, text},
};

use crate::Message;

// Placeholder icon colors per app type
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

pub fn app_item<'a>(
    index: usize,
    name: &'a str,
    icon: &'a str,
    hovered: bool,
    size: u32,
) -> Element<'a, Message> {
    let scale = if hovered { 1.2 } else { 1.0 };
    let icon_px = (size as f32 * scale) as u32;
    let color = icon_color(icon);

    let icon_widget = container(text(icon_label(icon)).size(icon_px / 2).color(Color::WHITE))
        .width(icon_px)
        .height(icon_px)
        .style(move |_theme| iced::widget::container::Style {
            background: Some(iced::Background::Color(color)),
            border: iced::Border {
                radius: (icon_px as f32 * 0.22).into(),
                ..Default::default()
            },
            ..Default::default()
        })
        .align_x(Alignment::Center)
        .align_y(Alignment::Center);

    let tooltip = if hovered {
        text(name).size(12).color(Color::WHITE)
    } else {
        text("").size(12)
    };

    let item = column![tooltip, icon_widget]
        .align_x(Alignment::Center)
        .spacing(4)
        .width(Length::Shrink);

    mouse_area(item)
        .on_enter(Message::AppHovered(index))
        .on_exit(Message::AppLeft)
        .on_press(Message::AppClicked(index))
        .into()
}

fn icon_label(icon: &str) -> &str {
    match icon {
        "folder" => "",
        "terminal" => ">_",
        "settings" => "⚙",
        "browser" => "◉",
        "editor" => "✎",
        _ => "?",
    }
}
