use iced::{
    Alignment, Background, Border, Color, Element, Length,
    widget::{container, row},
};

use crate::{Message, app_item::app_item, config::DockConfig};

const DOCK_BG: Color = Color {
    r: 0.12,
    g: 0.12,
    b: 0.14,
    a: 0.82,
};

pub fn dock_view(config: &DockConfig, hovered_app: Option<usize>) -> Element<Message> {
    let items: Vec<Element<Message>> = config
        .apps
        .iter()
        .enumerate()
        .map(|(i, app)| {
            app_item(
                i,
                &app.name,
                &app.icon,
                hovered_app == Some(i),
                config.icon_size,
            )
        })
        .collect();

    let dock_row = row(items)
        .align_y(Alignment::End)
        .spacing(config.gap)
        .padding([config.padding as u16, (config.padding + 6) as u16]);

    container(dock_row)
        .style(|_theme| iced::widget::container::Style {
            background: Some(Background::Color(DOCK_BG)),
            border: Border {
                radius: 18.0.into(),
                color: Color {
                    r: 1.0,
                    g: 1.0,
                    b: 1.0,
                    a: 0.08,
                },
                width: 1.0,
            },
            ..Default::default()
        })
        .width(Length::Shrink)
        .height(Length::Shrink)
        .into()
}
