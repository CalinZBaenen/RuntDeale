use graphics::types::{ColorComponent, Color};







pub const TRANSPARENT:Color = [0., 0., 0., 0.];
pub const YELLOW:Color =      [1., 1., 0., 1.];
pub const BLACK:Color =       [0., 0., 0., 1.];
pub const GREEN:Color =       [0., 1., 0., 1.];
pub const WHITE:Color =       [1., 1., 1., 1.];
pub const BLUE:Color =        [0., 0., 1., 1.];
pub const RED:Color =         [1., 0., 0., 1.];

pub fn rgba_to_color(r:u8, g:u8, b:u8, a:u8) -> Color {
	[
		ColorComponent::from(r)/255.,
		ColorComponent::from(g)/255.,
		ColorComponent::from(b)/255.,
		ColorComponent::from(a)/255.
	]
}
pub fn rgb_to_color(r:u8, g:u8, b:u8) -> Color {
	rgba_to_color(r, g, b, 255)
}