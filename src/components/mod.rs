pub mod marker {
	use bevy::ecs::component::Component;
	
	
	
	
	
	#[derive(Component, PartialEq, Clone, Debug, Copy, Eq)]
	pub struct Primary;
}





pub mod actor {
	use crate::components::plane::{Direction, Bounds};
	
	use bevy::ecs::component::Component;
	use bevy::sprite::SpriteSheetBundle;
	use bevy::ecs::bundle::Bundle;
	
	
	
	
	
	#[derive(Bundle)]
	pub struct PlayerBundle {
		pub(crate) attributes:EntityBundle,
		pub(crate) direction:Direction,
		pub(crate) player:Player
	}
	
	
	
	#[derive(Bundle)]
	pub struct EntityBundle {
		pub(crate) sprites:SpriteSheetBundle,
		pub(crate) bounds:Bounds
	}
	
	
	
	#[derive(Component, Clone, Copy)]
	pub struct Player;
}





pub mod plane {
	use crate::geometry::Rectangle;
	
	use bevy::ecs::component::Component;
	
	use std::convert::From;
	use std::fmt;
	
	
	
	
	
	#[repr(usize)]
	#[derive(Component, PartialEq, Clone, Debug, Copy, Eq)]
	pub enum Direction {
		North = 2,
		South = 0,
		East  = 3,
		West  = 1
	}
	
	impl Direction {
		pub fn from_strongest(x:f32, y:f32) -> Option<Self> {
			let (xa, ya) = (x.abs(), y.abs());
			if xa > ya && x < 0. { return Some(Self::West); }
			if xa > ya && x > 0. { return Some(Self::East); }
			if xa < ya && y < 0. { return Some(Self::South); }
			if xa < ya && y > 0. { return Some(Self::North); }
			None
		}
	}
	
	impl Default for Direction {
		fn default() -> Self { Self::South }
	}
	
	impl fmt::Display for Direction {
		fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
			write!(f, "{}", match *self {
				Self::North => "north",
				Self::South => "south",
				Self::East  => "east",
				Self::West  => "west"
			})
		}
	}
	
	
	
	#[derive(Component, PartialEq, Default, Clone, Debug, Copy)]
	pub struct Bounds {
		pub(crate) physical_bounds:Rectangle,
		pub(crate) texture_bounds:Option<Rectangle>,
		pub(crate) solid:bool
	}

	impl Bounds {
		pub fn with_texture_bounds(physical_bounds:Rectangle, texture_bounds:Rectangle) -> Self {
			Self {
				physical_bounds,
				texture_bounds:Some(texture_bounds),
				solid:false
			}
		}
		pub fn new(physical_bounds:Rectangle) -> Self {
			Self {
				physical_bounds,
				..Default::default()
			}
		}
		
		pub fn shift(&mut self, x:f32, y:f32) -> (f32, f32) {
			self.physical_bounds.0.0 += x;
			self.physical_bounds.0.1 += y;
			(self.physical_bounds.0.0, self.physical_bounds.0.1)
		}
	}

	impl From<(f32, f32, f32, f32)> for Bounds {
		fn from(bounds:(f32, f32, f32, f32)) -> Self {
			Self {
				physical_bounds:bounds.into(),
				..Default::default()
			}
		}
	}
	
	
	
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
}