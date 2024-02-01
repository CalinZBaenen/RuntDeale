use crate::components::{
	entity::{Direction, Bounds},
	general::Primary
};

use bevy::transform::components::{GlobalTransform, Transform};
use bevy::ecs::system::{SystemParam, Query};
use bevy::ecs::query::{WorldQuery, Has};
use bevy::sprite::TextureAtlasSprite;
use bevy::render::camera::Camera;





#[derive(WorldQuery)]
#[world_query(mutable)]
pub(crate) struct GeneralEntityQuery {
	pub(crate) direction:Option<&'static mut Direction>,
	pub(crate) transform:&'static mut Transform,
	pub(crate) bounds:&'static mut Bounds
}



#[derive(WorldQuery)]
#[world_query(mutable)]
pub(crate) struct EntityWithSpritesheetQuery {
	pub(crate) ta_sprite:&'static mut TextureAtlasSprite,
	pub(crate) general:GeneralEntityQuery
}



#[derive(SystemParam)]
pub(crate) struct CameraEffectParams<'w, 's> {
	pub(crate) camq:Query<'w, 's, (&'static Camera, &'static mut Transform, &'static GlobalTransform, Has<Primary>)>
}