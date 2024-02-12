pub mod texture {
	use bevy::sprite::{SpriteSheetBundle, SpriteBundle};
	use bevy::ecs::component::Component;
	
	use std::convert::From;
	
	
	
	
	
	#[derive(Component, Clone)]
	pub enum Texture {
		HasSpritesheet,
		Spritesheet(SpriteSheetBundle),
		HasSprite,
		Sprite(SpriteBundle)
	}
	
	impl From<SpriteSheetBundle> for Texture {
		fn from(ss:SpriteSheetBundle) -> Self { Self::Spritesheet(ss) }
	}
	
	impl From<SpriteBundle> for Texture {
		fn from(s:SpriteBundle) -> Self { Self::Sprite(s) }
	}
}





pub mod marker {
	use bevy::ecs::component::Component;
	
	
	
	
	
	/// Marks an entity as the primary one for its genera.
	#[derive(Component, PartialEq, Clone, Debug, Copy, Eq)]
	pub struct Primary;
	
	
	
	/// Marks an entity as a playable thing.
	#[derive(Component, Clone, Copy)]
	pub struct Player;
}





pub mod actor {
	use bevy::ecs::component::Component;
	
	
	
	
	
	#[derive(Component, Clone, Copy)]
	pub struct PlayerConfig {
		pub can_sprint:bool,
		pub upsidedown:bool,
		pub can_move:bool,
		pub speed:f32,
	}
	
	impl Default for PlayerConfig {
		fn default() -> Self {
			Self {
				can_sprint:true,
				upsidedown:false,
				can_move:true,
				speed:1.
			}
		}
	}
}





pub mod plane {
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
		pub(crate) bounds:(f32, f32, f32, f32),
		pub(crate) solid:bool
	}

	impl Bounds {
		/// Creates the `Bounds` from a 'flat' set of points.
		pub fn flat(a:f32, b:f32, c:f32, d:f32) -> Self { Self::new((a, b), c, d) }
		/// Creates the `Bounds` with default values for all except the physical bounds.
		pub fn new(loc:(f32, f32), width:f32, height:f32) -> Self {
			Self {
				bounds:(loc.0, loc.1, width, height),
				solid:false
			}
		}
		
		/// Moves the location of the contained rectangle by a
		/// specified amount and returns the new location.
		pub fn shift(&mut self, x:f32, y:f32) -> (f32, f32) {
			self.bounds.0 += x;
			self.bounds.1 += y;
			(self.bounds.0, self.bounds.1)
		}
	
		pub fn intersects(&self, other:&Self) -> bool {
			self.right().min(other.right()) > self.bounds.0.max(other.bounds.0) &&
			self.bottom().min(other.bottom()) > self.bounds.1.max(other.bounds.1)
		}
		pub fn bottom(&self) -> f32 { self.bounds.1+self.bounds.3 }
		pub fn right(&self) -> f32 { self.bounds.0+self.bounds.2 }
		pub fn area(&self) -> f32 { self.bounds.2*self.bounds.3 }
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