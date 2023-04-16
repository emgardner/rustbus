use iced::widget::button;
use iced::widget::container;
use iced::{Background, Color, Vector};

pub const COLUMN_SPACING: f32 = 20.0;
pub const ROW_SPACING: f32 = 20.0;
pub const HEADER_HEIGHT: f32 = 50.0;
pub const TEXT_COLOR: Color = Color { r: 0.94, g: 0.94, b: 0.94, a: 1.0 };
pub const BACKGROUND_COLOR: Color = Color { r: 0.13, g: 0.13, b: 0.13, a: 1.0 };
pub const BACKGROUND_COLOR_HOVERED: Color = Color { r: 0.5, g: 0.5, b: 0.5, a: 1.0 };
pub const BORDER_BUTTON_RADIUS: f32 = 2.0;


pub enum ButtonType {
    Image
}

impl From<ButtonType> for iced::theme::Button {
    fn from(button_type: ButtonType) -> Self {
        iced::theme::Button::Custom(Box::new(button_type))
    }
}

impl button::StyleSheet for ButtonType {
     type Style = iced::Theme;

    fn active(&self, _: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: None,
            border_radius: 0.0,
            border_width: 0.0,
            shadow_offset: Vector::new(0.0, 0.0),
            text_color: TEXT_COLOR,
            border_color: Color::from_rgba8(0, 0, 0, 0.0),
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        // button::StyleSheet::hovered(self, style)
        button::Appearance {
            shadow_offset: self.active(style).shadow_offset + Vector::new(0.0, 1.0),
            background: Some(Background::Color(BACKGROUND_COLOR_HOVERED)),
            border_radius: BORDER_BUTTON_RADIUS,
            border_width: 0.0,
            border_color: TEXT_COLOR,
            text_color: TEXT_COLOR
        }
    }

    fn disabled(&self, style: &Self::Style) -> button::Appearance {
        button::StyleSheet::active(self, style)
    }

}



#[derive(Debug, Copy, Clone)]
pub struct ContainerStyle {
    pub text_color: Option<Color>,
    pub background: Option<Background>,
    pub border_radius: f32,
    pub border_width: f32,
    pub border_color: Color,
}

impl From<ContainerStyle> for container::Appearance {
    fn from(cs: ContainerStyle) -> container::Appearance {
        container::Appearance {
            text_color: cs.text_color,
            background: cs.background,
            border_radius: cs.border_radius,
            border_width: cs.border_width,
            border_color: cs.border_color,
        }
    }
}

impl container::StyleSheet for ContainerStyle {
    type Style = iced::Theme;
    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance::from(*self)
    }
}
