use bevy::ecs::component::Component;

use std::ops::{Add, Sub};
use std::fmt;





#[derive(Component, PartialEq, Clone, Debug, Copy)]
pub struct Location {
	position:(f32, f32),
	layer:u8
}

impl Location {
	pub fn new(x:f32, y:f32) -> Self {
		Self {position:(x, y), layer:1}
	}
}

impl Default for Location {
	fn default() -> Self {
		Self {position:(0., 0.), layer:1}
	}
}

impl fmt::Display for Location {
	fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
		write!(f, "({}, {})", self.position.0, self.position.1)
	}
}

impl Add for Location {
	type Output = Self;
	
	fn add(self, rhs:Self) -> Self::Output {
		Self {
			position:(self.position.0+rhs.position.0, self.position.1+rhs.position.1),
			layer:self.layer
		}
	}
}

impl Sub for Location {
	type Output = Self;
	
	fn sub(self, rhs:Self) -> Self::Output {
		Self {
			position:(self.position.0-rhs.position.0, self.position.1-self.position.1),
			layer:self.layer
		}
	}
}



#[derive(Component, PartialEq, Clone, Debug, Copy, Eq)]
pub struct Primary;