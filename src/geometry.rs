use std::convert::From;
use std::fmt;





#[derive(PartialEq, Default, Clone, Debug, Copy)]
pub struct Rectangle(pub(crate) Point, pub(crate) Point);

impl Rectangle {
	pub fn new(a:f32, b:f32, c:f32, d:f32) -> Self { Self(Point(a, b), Point(c, d)) }
	
	pub fn intersects(&self, other:&Self) -> bool {
		self.right().min(other.right()) > self.0.0.max(other.0.0) &&
		self.bottom().min(other.bottom()) > self.0.1.max(other.0.1)
	}
	pub fn bottom(&self) -> f32 { self.0.1+self.1.1 }
	pub fn right(&self) -> f32 { self.0.0+self.1.0 }
	pub fn area(&self) -> f32 { self.1.0*self.1.1 }
}

impl From<(f32, f32, f32, f32)> for Rectangle {
	fn from(bounds:(f32, f32, f32, f32)) -> Self {
		Self::new(bounds.0, bounds.1, bounds.2, bounds.3)
	}
}



#[derive(PartialEq, Default, Clone, Debug, Copy)]
pub struct Point(pub(crate) f32, pub(crate) f32);

impl Point {
	pub fn new(a:f32, b:f32) -> Self { Point(a, b) }
}

impl fmt::Display for Point {
	fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
		write!(f, "({}, {})", self.0, self.1)
	}
}