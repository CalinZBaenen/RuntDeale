/// A generic representation of a rectangle.
#[derive(PartialEq, Default, Clone, Debug, Copy)]
pub struct Rectangle(pub(crate) f32, pub(crate) f32, pub(crate) f32, pub(crate) f32);

impl Rectangle {
	pub fn new(loc:(f32, f32), width:f32, height:f32) -> Self { Self(loc.0, loc.1, width, height) }
	
	pub fn intersects(&self, other:&Self) -> bool {
		self.right().min(other.right()) > self.0.max(other.0) &&
		self.bottom().min(other.bottom()) > self.1.max(other.1)
	}
	pub fn bottom(&self) -> f32 { self.1+self.3 }
	pub fn right(&self) -> f32 { self.0+self.2 }
	pub fn area(&self) -> f32 { self.2*self.3 }
}