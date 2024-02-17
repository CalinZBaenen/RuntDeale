use crate::components::{plane::{Direction, Bounds, Follow}, marker::{Primary, Player}};

use bevy::ecs::query::{WorldQuery, With, Has, Or};
use bevy::sprite::{TextureAtlasSprite, Sprite};
use bevy::transform::components::Transform;
use bevy::render::camera::Camera;





/// A query used to obtain entities with a texture.
#[derive(WorldQuery)]
#[world_query(mutable)]
pub struct PrimaryCameraQuery {
	/// Filter.
	    _filter:(With<Primary>, With<Follow>),
	/// The camera.
	pub camera:CameraQuery
}



impl CameraQueryItem<'_> {
	pub const fn is_primary(&self) -> bool { self.is_primary }
}



/// A query used to obtain entities with spatial metadata.
#[derive(WorldQuery)]
#[world_query(mutable)]
pub struct SpatialQuery {
	/// THe direction the entity is facing.
	pub direction:Option<&'static mut Direction>,
	/// The transformation of the entity.
	pub transform:&'static mut Transform,
	/// The bounds that the entity lies within.
	pub bounds:Option<&'static mut Bounds>,
	/// The axes that the entity should follow (the player) along.
	pub follow:Option<&'static mut Follow>,
}



/// A query used to obtain entities with a texture.
#[derive(WorldQuery)]
#[world_query(mutable)]
pub struct TextureQuery {
	/// The potential `TextureAtlasSprite` associated with this entity.
	pub ta_sprite:Option<&'static mut TextureAtlasSprite>,
	/// Filter.
	    _filter:Or<(With<TextureAtlasSprite>, With<Sprite>)>,
	/// The potential `Sprite` associated with this entity.
	pub sprite:Option<&'static mut Sprite>
}



/// A query used to obtain entities that are cameras.
#[derive(WorldQuery)]
#[world_query(mutable)]
pub struct CameraQuery {
	/// The spatial configuration of the camera.
	pub spatial_config:SpatialQuery,
	/// Whether or not the camera is primary.
	    is_primary:Has<Primary>,
	/// The `Camera` component.
	pub camera:&'static Camera
}



/// A query used to obtain information about the player(s).
#[derive(WorldQuery)]
#[world_query(mutable)]
pub struct PlayerQuery {
	/// Filter.
	    _filter:(With<Bounds>, With<Player>),
	/// The actor associated with the player.
	pub actor:ActorQuery
}



/// A query used to obtain entities with some general (actor) components, such as `Bounds`.
#[derive(WorldQuery)]
#[world_query(mutable)]
pub struct ActorQuery {
	/// The spatial configuration of the entity.
	pub spatial_config:SpatialQuery,
	/// The underlying `TextureQuery`.
	pub texture:TextureQuery,
	/// Filter.
	    _filter:With<Bounds>
}