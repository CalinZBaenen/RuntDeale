use crate::geometry::Rectangle;

use bevy::ecs::component::Component;
use bevy::sprite::SpriteSheetBundle;
use bevy::ecs::bundle::Bundle;

use std::convert::From;





#[derive(Bundle)]
pub struct EntityBundle {
	pub(crate) sprites:SpriteSheetBundle,
	pub(crate) bounds:Bounds
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
	
	pub fn shift(&mut self, mut x:f32, mut y:f32) -> (f32, f32) {
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