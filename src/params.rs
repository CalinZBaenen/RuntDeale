use crate::components::{
	entity::{Direction, Bounds},
	general::{Primary, Follow}
};

use bevy::transform::components::Transform;
use bevy::ecs::query::{WorldQuery, Has};
use bevy::sprite::TextureAtlasSprite;
use bevy::render::camera::Camera;





#[derive(WorldQuery)]
#[world_query(mutable)]
pub struct GeneralEntityQuery {
	pub direction:Option<&'static mut Direction>,
	pub transform:&'static mut Transform,
	pub bounds:&'static mut Bounds
}



#[derive(WorldQuery)]
#[world_query(mutable)]
pub struct EntityWithSpritesheetQuery {
	pub ta_sprite:&'static mut TextureAtlasSprite,
	pub general:GeneralEntityQuery
}



#[derive(WorldQuery)]
#[world_query(mutable)]
pub struct CameraQuery {
	pub is_primary:Has<Primary>,
	pub transform:&'static mut Transform,
	pub camera:&'static Camera,
	pub follow:Option<&'static Follow>
}