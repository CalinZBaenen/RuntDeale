use crate::params::{EntityWithSpritesheetQuery, CameraQuery};
use crate::components::{entity::Direction, player::Player};

use bevy::ecs::{
	query::{Without, With},
	system::{Query, Res}
};
use bevy::input::{keyboard::KeyCode, Input};
use bevy::math::Vec2;





pub(crate) fn sys_handle_freeroaming_controls(
	mut cameras:Query<CameraQuery, Without<Player>>,
	mut players:Query<EntityWithSpritesheetQuery, With<Player>>,
	keys:Res<Input<KeyCode>>
) {
	let (mut mx, mut my) = (0., 0.);
	
	for &key in keys.get_pressed() {
		match key {
			KeyCode::Right | KeyCode::D => { mx += 1.; }
			KeyCode::Down | KeyCode::S => { my -= 1.; }
			KeyCode::Left | KeyCode::A => { mx -= 1.; }
			KeyCode::Up | KeyCode::W => { my += 1.; }
			_ => {}
		}
	}
	if keys.pressed(KeyCode::ControlLeft) {
		mx *= 2.;
		my *= 2.;
	}
	
	if mx > 0. && my > 0. {
		mx *= 0.7;
		my *= 0.7;
	}
	
	for mut pq in &mut players {
		let mut attrs = pq.general;
		
		let dir = Direction::from_strongest(mx, my).unwrap_or(**attrs.direction.as_ref().unwrap());
		let (nx, ny) = attrs.bounds.shift(mx, my);
		
		attrs.transform.translation.x = nx;
		attrs.transform.translation.y = ny;
		*attrs.direction.unwrap()     = dir;
		
		pq.ta_sprite.index = 3*(dir as usize);
	};
	
	for mut camera in &mut cameras {
		if !camera.is_primary { continue; }
		if let Some(translation) = camera.camera.viewport_to_world_2d(camera.global_transform, Vec2::splat(0.0)) {
			let translation = Vec2::new(translation.x*mx, translation.y*my);
			camera.transform.translation = (1.0 * translation).extend(camera.transform.translation.z);
		}
	}
}