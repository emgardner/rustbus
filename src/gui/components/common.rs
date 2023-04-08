use iced::theme::Theme;
use iced::widget::{image, image::Handle, row, text, Container, button};
use iced::Element;
use iced::Renderer;
use iced::{Background, Color, Length};
use crate::gui::protocol::Protocol;
use crate::gui::style::ButtonType;
use iced_native::widget::container::Appearance;
use iced_native::widget::container::StyleSheet;

#[derive(Debug, Copy, Clone)]
pub struct ContainerStyle {
    pub text_color: Option<Color>,
    pub background: Option<Background>,
    pub border_radius: f32,
    pub border_width: f32,
    pub border_color: Color,
}

impl From<ContainerStyle> for Appearance {
    fn from(cs: ContainerStyle) -> Appearance {
        Appearance {
            text_color: cs.text_color,
            background: cs.background,
            border_radius: cs.border_radius,
            border_width: cs.border_width,
            border_color: cs.border_color,
        }
    }
}

impl StyleSheet for ContainerStyle {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> Appearance {
        Appearance::from(*self)
    }
}

pub fn screen<'a, T, Message>(t: T) -> Container<'a, Message, Renderer>
where
    T: Into<Element<'a, Message, Renderer>>,
{
    Container::new(t)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
}

pub fn header() -> Element<'static, Protocol> {
    Container::new(
        row![
            image(Handle::from_path("./resources/modbus_logo.png"))
                .height(45.0)
                .content_fit(iced_native::ContentFit::Contain),
            row![text("RustBus")
                .width(Length::Fill)
                .horizontal_alignment(iced::alignment::Horizontal::Center)
                .size(20.0)]
            .align_items(iced::Alignment::Center)
            .width(Length::Fill),
            row![
            button(
            image(Handle::from_path("./resources/folder.png"))
                .height(45.0)
                .content_fit(iced_native::ContentFit::Contain),
            ).style(ButtonType::Image.into()).on_press(Protocol::OpenFileDialog),
            button(
            image(Handle::from_path("./resources/diskette.png"))
                .height(45.0)
                .content_fit(iced_native::ContentFit::Contain),
            ).style(ButtonType::Image.into()).on_press(Protocol::SaveFileDialog)
            ].spacing(20),
            // Container::new(text("")).width(127.0).height(Length::Fill)
        ]
        .align_items(iced::Alignment::Center),
    )
    .width(Length::Fill)
    .height(Length::Fixed(60.0))
    .center_x()
    .center_y()
    .padding(10.0)
    .into()
}
