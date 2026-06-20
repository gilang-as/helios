use cosmic::Element;
use cosmic::app::{Application, Core, Settings, Task};
use cosmic::iced::platform_specific::runtime::wayland::layer_surface::SctkLayerSurfaceSettings;
use cosmic::iced::platform_specific::shell::wayland::commands::layer_surface::{
    Anchor, KeyboardInteractivity, Layer, get_layer_surface,
};
use cosmic::iced::window;
use cosmic::iced::{Alignment, Background, Color, Length};
use cosmic::widget::{container, text};

static LAYER_ID: std::sync::LazyLock<window::Id> = std::sync::LazyLock::new(window::Id::unique);

fn main() -> cosmic::iced::Result {
    cosmic::app::run::<HeliosBar>(Settings::default().no_main_window(true), ())
}

struct HeliosBar {
    core: Core,
}

#[derive(Debug, Clone)]
pub enum Message {}

impl Application for HeliosBar {
    type Executor = cosmic::executor::Default;
    type Flags = ();
    type Message = Message;
    const APP_ID: &'static str = "io.helios.Bar";

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
            anchor: Anchor::TOP | Anchor::LEFT | Anchor::RIGHT,
            size: Some((None, Some(30))),
            exclusive_zone: 30,
            ..Default::default()
        });
        (Self { core }, task)
    }

    fn update(&mut self, _message: Message) -> Task<Message> {
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        cosmic::widget::text("").into()
    }

    fn view_window(&self, id: window::Id) -> Element<'_, Message> {
        if id != *LAYER_ID {
            return cosmic::widget::text("").into();
        }
        container(text("Helios Bar").size(13.0).class(Color::WHITE))
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(
                |_theme: &cosmic::Theme| cosmic::iced::widget::container::Style {
                    background: Some(Background::Color(Color::from_rgba(0.08, 0.08, 0.08, 0.9))),
                    ..Default::default()
                },
            )
            .into()
    }
}
