pub mod marker {
	use bevy::ecs::component::Component;
	
	
	
	
	
	/// Marks an entity as the primary one for its genera.
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
	
	
	
	/// Marks an entity as a playable thing.
	#[derive(Component, Clone, Copy)]
	pub struct Player;
}





pub mod plane {
	use crate::geometry::Rectangle;
	
	use bevy::ecs::component::Component;
	
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
	
	
	
	/// Represents the bounds that an entity lies within.
	#[derive(Component, PartialEq, Default, Clone, Debug, Copy)]
	pub struct Bounds {
		pub(crate) bounds:Rectangle,
		pub(crate) solid:bool
	}

	impl Bounds {
		/// Creates the `Bounds` from a 'flat' set of points.
		pub fn flat(a:f32, b:f32, c:f32, d:f32) -> Self {
			Self::new(Rectangle::new((a, b), c, d))
		}
		/// Creates the `Bounds` with default values for all except the physical bounds.
		pub fn new(bounds:Rectangle) -> Self {
			Self {
				bounds,
				..Default::default()
			}
		}
		
		/// Moves the location of the contained rectangle by a
		/// specified amount and returns the new location.
		pub fn shift(&mut self, x:f32, y:f32) -> (f32, f32) {
			self.bounds.0 += x;
			self.bounds.1 += y;
			(self.bounds.0, self.bounds.1)
		}
	}
	
	
	
	/// Determines in what directions something will follow another.
	#[derive(Component, PartialEq, Clone, Debug, Copy, Eq)]
	pub enum Follow {
		/// Follows on the X-axis.
		Horizontal,
		/// Follows on the Y-axis.
		Vertical,
		/// Follows on neither axes.
		Neither,
		/// Follows on both axes.
		Both
	}
	
	impl Follow {
		pub const DEFAULT:Self = Self::Both;
		
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