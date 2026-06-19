mod app_item;
mod config;
mod dock;

use config::DockConfig;
use dock::dock_view;
use iced::{Alignment, Color, Element, Length, Theme, widget::container};

fn main() -> iced::Result {
    iced::application(Helios::new, Helios::update, Helios::view)
        .title("Helios Dock")
        .theme(Helios::theme)
        .transparent(true)
        .decorations(false)
        .run()
}

struct Helios {
    config: DockConfig,
    hovered_app: Option<usize>,
}

impl Helios {
    fn new() -> (Self, iced::Task<Message>) {
        (
            Self {
                config: DockConfig::default(),
                hovered_app: None,
            },
            iced::Task::none(),
        )
    }
}

#[derive(Debug, Clone)]
enum Message {
    AppHovered(usize),
    AppLeft,
    AppClicked(usize),
}

impl Helios {
    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::AppHovered(i) => self.hovered_app = Some(i),
            Message::AppLeft => self.hovered_app = None,
            Message::AppClicked(i) => {
                if let Some(app) = self.config.apps.get(i) {
                    println!("Launched: {}", app.name);
                }
            }
        }
        iced::Task::none()
    }

    fn view(&self) -> Element<Message> {
        let dock = dock_view(&self.config, self.hovered_app);

        // Center dock at bottom of window
        container(
            container(dock)
                .align_x(Alignment::Center)
                .align_y(Alignment::End)
                .width(Length::Fill)
                .height(Length::Fill),
        )
        .style(|_| iced::widget::container::Style {
            background: Some(iced::Background::Color(Color::TRANSPARENT)),
            ..Default::default()
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}
