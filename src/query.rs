use crate::components::{plane::{Direction, Bounds, Follow}, marker::Primary};

use bevy::sprite::{TextureAtlasSprite, Sprite};
use bevy::transform::components::Transform;
use bevy::ecs::query::{WorldQuery, Has};
use bevy::render::camera::Camera;





/// A query used to obtain entities that have a spritesheet.  
/// (Unintuitively made to accomodate Bevy's distinction between `Sprite` and `TextureAtlasSprite`.)
#[derive(WorldQuery)]
#[world_query(mutable)]
pub struct ActorWithTextureQuery {
	pub texture:TextureQuery,
	pub actor:ActorQuery
}



/// A query used to obtain entities with a texture.
#[derive(WorldQuery)]
#[world_query(mutable)]
pub struct TextureQuery {
	pub ta_sprite:Option<&'static mut TextureAtlasSprite>,
	pub sprite:Option<&'static mut Sprite>
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



/// A query used to obtain entities with some general (actor) components, such as `Bounds`.
#[derive(WorldQuery)]
#[world_query(mutable)]
pub struct ActorQuery {
	pub direction:Option<&'static mut Direction>,
	pub transform:&'static mut Transform,
	pub bounds:&'static mut Bounds
}