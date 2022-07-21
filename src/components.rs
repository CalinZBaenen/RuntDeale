use std::default::Default;
use bevy::prelude::*;





/// "Float Clarity"; the precision of floats.
pub type FltClrty = f32;

/// The direction an orientated component is facing.
#[derive(PartialEq, Clone, Copy, Eq)]
pub enum Facing {North, East, South, West}



/// `Directional` component.
/// Specifies an entity has an orientation.
#[derive(PartialEq, Component, Clone, Copy, Eq)]
pub struct Directional(pub Facing);

/// A marker component that should only be put
/// on a single camera entity.  
/// This is meant for fetching the display.
#[derive(Component)]
pub struct MainCamera;

/// `Location` component.  
/// Represents a point in 2D space
/// represented as a X, Y, and layer coordinate triplet.
#[derive(Component, PartialEq, Clone)]
pub struct Location(pub FltClrty, pub FltClrty, pub u8);

/// The amount of health an entity has.
#[derive(PartialOrd, Component, PartialEq, Clone, Copy, Ord, Eq<u32>, Eq)]
pub struct Health(pub u32);
impl PartialEq<u32> for Health {
	fn eq(&self, other:&u32) -> bool { self.0 == *other }
}

/// `Player` component.  
/// Specifies an entity is a player
/// (and thusly can be controlled when allowed).
#[derive(Component, Clone)]
pub struct Player {
	level:u8,
	love:u32,
	xp:u16
}

/// Whether an entity is solid or not.
#[derive(Component, PartialEq, Clone, Copy, Eq)]
pub struct Solid(pub bool);

/// An entity's name.
#[derive(Component, PartialEq, Clone, Eq)]
pub struct Name(pub String);
impl Default for Name {
	fn default() -> Self { Self(String::new()) }
}

/// `NPC` component.
/// Specifies an entity is an NPC.
#[derive(Component)]
pub struct NPC;



/// A bundle of position-related components.
#[derive(Bundle)]
pub struct PositionBundle {
	pub location:Point,
	pub layer:Layer
}

/// A bundle that represents a player.
#[derive(Bundle)]
pub struct PlayerBundle {
	pub directional:Directional,
	pub health:Health,
	pub player:Player,
	
	#[bundle] pub position:PositionBundle,
	#[bundle] pub sprites:SpriteSheetBundle
}