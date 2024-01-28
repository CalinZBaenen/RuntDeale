use crate::components::general::Primary;

use bevy::transform::components::{GlobalTransform, Transform};
use bevy::ecs::system::{SystemParam, Query};
use bevy::render::camera::Camera;
use bevy::ecs::query::Has;
use bevy::math::Vec2;





#[derive(SystemParam)]
pub struct CameraEffectParams<'w, 's> {
	camq:Query<'w, 's, (&'static Camera, &'static mut Transform, &'static GlobalTransform, Has<Primary>)>
}





pub(crate) fn sys_edit_camera(mut params:CameraEffectParams) {
	for (camera, mut transform, global_transform, is_primary) in &mut params.camq {
		if !is_primary { continue; }
		if let Some(translation) = camera.viewport_to_world_2d(global_transform, Vec2::splat(0.0)) {
			println!("{}", translation);
			transform.translation = (-1.0 * translation).extend(transform.translation.z);
		}
	}
}



pub(crate) fn camera_exists(q:Query<&Camera>) -> bool { q.iter().len() > 0 }