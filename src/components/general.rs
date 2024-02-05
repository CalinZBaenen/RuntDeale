use bevy::ecs::component::Component;





#[derive(Component, PartialEq, Clone, Debug, Copy, Eq)]
pub struct Primary;



#[derive(Component, PartialEq, Clone, Debug, Copy, Eq)]
pub enum Follow {
	Horizontal,
	Vertical,
	Neither,
	Both
}

impl Follow {
	pub const fn horizontal(self) -> bool {
		match self {
			Self::Horizontal | Self::Both => true,
			_ => false
		}
	}
	pub const fn vertical(self) -> bool {
		match self {
			Self::Vertical | Self::Both => true,
			_ => false
		}
	}
}