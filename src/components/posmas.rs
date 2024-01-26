use bevy::ecs::component::Component;

use std::ops::{Add, Sub};
use std::fmt;





#[derive(Component, PartialEq, Clone, Debug, Copy, Eq)]
pub struct Location {
	position:(usize, usize),
	layer:u8
}

impl Location {
	fn new(x:usize, y:usize) -> Self {
		Self {position:(x, y), layer:1}
	}
}

impl Default for Location {
	fn default() -> Self {
		Self {position:(0, 0), layer:1}
	}
}

impl fmt::Display for Location {
	fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
		write!(f, "")
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