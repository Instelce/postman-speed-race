use bevy::{
    color::palettes::tailwind::{GRAY_300, INDIGO_400, INDIGO_500, INDIGO_600},
    prelude::*,
};

pub const BUTTON_HOVERED_BACKGROUND: Color = Color::Srgba(INDIGO_500);
pub const BUTTON_PRESSED_BACKGROUND: Color = Color::Srgba(INDIGO_600);

pub const BUTTON_TEXT: Color = Color::srgb(0.925, 0.925, 0.925);
pub const LABEL_TEXT: Color = Color::Srgba(GRAY_300);
pub const HEADER_TEXT: Color = Color::srgb(0.867, 0.827, 0.412);

pub const NODE_BACKGROUND: Color = Color::Srgba(INDIGO_400);

pub const BACKGROUND: Color = Color::srgb(0.471, 0.765, 0.286);
