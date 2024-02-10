use crate::components::{plane::{Direction, Bounds, Follow}, marker::Primary};

use bevy::transform::components::Transform;
use bevy::ecs::query::{WorldQuery, Has};
use bevy::sprite::TextureAtlasSprite;
use bevy::render::camera::Camera;





/// A query used to obtain entities with some general (actor) components, such as `Bounds`.
#[derive(WorldQuery)]
#[world_query(mutable)]
pub struct ActorQuery {
	pub direction:Option<&'static mut Direction>,
	pub transform:&'static mut Transform,
	pub bounds:&'static mut Bounds
}



/// A query used to obtain entities that have a spritesheet.  
/// (Unintuitively made to accomodate Bevy's distinction between `Sprite` and `TextureAtlasSprite`.)
#[derive(WorldQuery)]
#[world_query(mutable)]
pub struct ActorWithSpritesheetQuery {
	pub ta_sprite:&'static mut TextureAtlasSprite,
	pub general:ActorQuery
}



/// A query used to obtain entities that are cameras.
#[derive(WorldQuery)]
#[world_query(mutable)]
pub struct CameraQuery {
	pub is_primary:Has<Primary>,
	pub transform:&'static mut Transform,
	pub camera:&'static Camera,
	pub follow:Option<&'static Follow>
}