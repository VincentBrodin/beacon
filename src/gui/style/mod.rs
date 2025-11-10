use iced::{
    Border, Color, Shadow,
    widget::container::{Appearance, StyleSheet},
};

#[derive(Default)]
pub(crate) struct ItemContainer;

#[derive(Default)]
pub(crate) enum ItemContainerStyle {
    #[default]
    Default,
    Highlighted,
}

impl StyleSheet for ItemContainer {
    type Style = ItemContainerStyle;

    fn appearance(&self, style: &Self::Style) -> Appearance {
        match style {
            ItemContainerStyle::Default => Appearance {
                text_color: None,
                background: None,
                border: Border::default(),
                shadow: Shadow::default(),
            },
            ItemContainerStyle::Highlighted => Appearance {
                text_color: None,
                background: Some(iced::Background::Color(Color::BLACK)),
                border: Border::default(),
                shadow: Shadow::default(),
            },
        }
    }
}
