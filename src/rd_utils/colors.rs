#![allow(dead_code)]
use graphics::types::{Color, ColorComponent};

pub const RED: Color = [1.0, 0.0, 0.0, 1.0];
pub const YELLOW: Color = [1.0, 1.0, 0.0, 1.0];
pub const GREEN: Color = [0.0, 1.0, 0.0, 1.0];
pub const CYAN: Color = [0.0, 1.0, 1.0, 1.0];
pub const BLUE: Color = [0.0, 0.0, 1.0, 1.0];
pub const MAGENTA: Color = [1.0, 0.0, 1.0, 1.0];
pub const TRANSPARENT: Color = [0.0, 0.0, 0.0, 0.0];
pub const BLACK: Color = [0.0, 0.0, 0.0, 1.0];
pub const WHITE: Color = [1.0, 1.0, 1.0, 1.0];

pub fn rgba_to_normalized(r: u8, g: u8, b: u8, a: u8) -> Color {
    [
        ColorComponent::from(r) / 255.0,
        ColorComponent::from(g) / 255.0,
        ColorComponent::from(b) / 255.0,
        ColorComponent::from(a) / 255.0,
    ]
}
pub fn rgb_to_normalized(r: u8, g: u8, b: u8) -> Color {
    rgba_to_normalized(r, g, b, 255)
}
