use crate::components::{general::Primary, entity::Bounds};

use bevy::transform::components::{GlobalTransform, Transform};
use bevy::ecs::query::{ReadOnlyWorldQuery, Has};
use bevy::ecs::system::{SystemParam, Query};
use bevy::render::camera::Camera;





#[derive(SystemParam)]
pub(crate) struct GeneralEntityParams<'w, 's, F:ReadOnlyWorldQuery+'static=()> {
	pub(crate) enq:Query<'w, 's, (&'static mut Bounds, &'static mut Transform), F>
}



#[derive(SystemParam)]
pub(crate) struct CameraEffectParams<'w, 's> {
	pub(crate) camq:Query<'w, 's, (&'static Camera, &'static mut Transform, &'static GlobalTransform, Has<Primary>)>
}