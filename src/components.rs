pub mod texture {
	use bevy::sprite::{SpriteSheetBundle, SpriteBundle};
	use bevy::ecs::component::Component;
	
	use std::convert::From;
	
	
	
	
	
	#[derive(Component, Clone)]
	pub enum Texture {
		Spritesheet(SpriteSheetBundle),
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
	#[derive(PartialEq, Component, Default, Clone, Debug, Copy, Eq)]
	pub(crate) struct Primary;
	
	
	
	/// Marks an entity as a playable thing.
	#[derive(PartialEq, Component, Default, Clone, Debug, Copy, Eq)]
	pub struct Player;
}





pub mod actor {
	use bevy::ecs::component::Component;
	
	
	
	
	
	/// Player statistics.
	#[derive(Component, Clone, Copy)]
	pub struct PlayerStats {
		pub upsidedown:bool,
		pub stamina:u32,
		pub level:u32,
		pub speed:f32
	}
	
	impl Default for PlayerStats {
		fn default() -> Self {
			Self {
				upsidedown:false,
				stamina:100,
				level:0,
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
		/// The default value of `Follow` expressed as a constant.
		pub const DEFAULT:Self = Self::Both;
		
		
		/// Returns the coordinates that something should be at if it follows
		/// particular axes, staying at the given coordinate if it does not
		/// move along that axis.
		pub fn follow_or(self, follow_to:(f32, f32), locked_at:(f32, f32)) -> (f32, f32) {
			(
				if self.horizontal() { follow_to.0 } else { locked_at.0 },
				if self.vertical() { follow_to.1 } else { locked_at.1 }
			)
		}
		/// Checks if this `Follow`-value allows for movement horizontally.
		pub const fn horizontal(self) -> bool {
			match self {
				Self::Horizontal | Self::Both => true,
				_ => false
			}
		}
		/// Checks if this `Follow`-value allows for movement vertically.
		pub const fn vertical(self) -> bool {
			match self {
				Self::Vertical | Self::Both => true,
				_ => false
			}
		}
	}
}