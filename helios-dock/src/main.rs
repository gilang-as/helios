mod app_item;
mod config;
mod dock;

use config::DockConfig;
use cosmic::Element;
use cosmic::app::{Application, Core, Settings, Task};
use cosmic::iced::platform_specific::runtime::wayland::layer_surface::SctkLayerSurfaceSettings;
use cosmic::iced::platform_specific::shell::wayland::commands::layer_surface::{
    Anchor, KeyboardInteractivity, Layer, get_layer_surface,
};
use cosmic::iced::window;
use cosmic::iced::{Alignment, Length};
use cosmic::widget::container;

static LAYER_ID: std::sync::LazyLock<window::Id> = std::sync::LazyLock::new(window::Id::unique);

fn main() -> cosmic::iced::Result {
    cosmic::app::run::<HeliosDock>(Settings::default().no_main_window(true), ())
}

struct HeliosDock {
    core: Core,
    config: DockConfig,
    hovered_app: Option<usize>,
}

#[derive(Debug, Clone)]
pub enum Message {
    AppHovered(usize),
    AppLeft,
    AppClicked(usize),
}

impl Application for HeliosDock {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = Message;
    const APP_ID: &'static str = "io.helios.Dock";

    fn core(&self) -> &Core {
        &self.core
    }

    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }

    fn init(core: Core, _flags: ()) -> (Self, Task<Message>) {
        let task = get_layer_surface(SctkLayerSurfaceSettings {
            id: *LAYER_ID,
            layer: Layer::Top,
            keyboard_interactivity: KeyboardInteractivity::None,
            anchor: Anchor::BOTTOM | Anchor::LEFT | Anchor::RIGHT,
            size: Some((None, Some(64))),
            exclusive_zone: 64,
            ..Default::default()
        });
        (
            Self {
                core,
                config: DockConfig::default(),
                hovered_app: None,
            },
            task,
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::AppHovered(i) => self.hovered_app = Some(i),
            Message::AppLeft => self.hovered_app = None,
            Message::AppClicked(i) => {
                if let Some(app) = self.config.apps.get(i) {
                    println!("Launched: {}", app.name);
                }
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        cosmic::widget::text("").into()
    }

    fn view_window(&self, id: window::Id) -> Element<'_, Message> {
        if id != *LAYER_ID {
            return cosmic::widget::text("").into();
        }
        let dock = dock::dock_view(&self.config, self.hovered_app);
        container(dock)
            .align_x(Alignment::Center)
            .align_y(Alignment::End)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
