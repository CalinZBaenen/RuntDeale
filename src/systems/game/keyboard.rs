use crate::components::{
	plane::{Direction, Follow},
	actor::Player
};
use crate::params::{ActorWithSpritesheetQuery, CameraQuery};

use bevy::ecs::{
	query::{Without, With},
	system::{Query, Res}
};
use bevy::input::{keyboard::KeyCode, Input};
use bevy::math::Vec2;





/// Handles the controls when the player should be in a `Roaming` state,
/// like on the map or in the battlebox.
pub(crate) fn sys_handle_freeroaming_controls(
	mut cameras:Query<CameraQuery, Without<Player>>,
	mut players:Query<ActorWithSpritesheetQuery, With<Player>>,
	keys:Res<Input<KeyCode>>
) {
	// Player movement.
	let mut mvmnt = Vec2::new(0., 0.);
	
	for &key in keys.get_pressed() {
		match key {
			KeyCode::Right | KeyCode::D => { mvmnt.x += 1.; }
			KeyCode::Down  | KeyCode::S => { mvmnt.y -= 1.; }
			KeyCode::Left  | KeyCode::A => { mvmnt.x -= 1.; }
			KeyCode::Up    | KeyCode::W => { mvmnt.y += 1.; }
			_ => {}
		}
	}
	
	mvmnt = mvmnt.normalize_or_zero();
	if keys.pressed(KeyCode::ControlLeft) { mvmnt *= 2.; }
	
	let (mut nx, mut ny) = (0., 0.);
	for mut pq in &mut players {
		let mut attrs = pq.general;
		
		let dir = Direction::from_strongest(mvmnt.x, mvmnt.y).unwrap_or(**attrs.direction.as_ref().unwrap());
		(nx, ny) = attrs.bounds.shift(mvmnt.x, mvmnt.y);
		
		attrs.transform.translation.x = nx;
		attrs.transform.translation.y = ny;
		*attrs.direction.unwrap()     = dir;
		
		pq.ta_sprite.index = 3*(dir as usize);
	};
	
	for mut camera in &mut cameras {
		if !camera.is_primary { continue; }
		
		let follow = camera.follow.unwrap_or(&Follow::DEFAULT);
		let translation = Vec2::new(
			if follow.horizontal() { nx } else { 0. },
			if follow.vertical() { ny } else { 0. }
		);
		
		camera.transform.translation = translation.extend(camera.transform.translation.z);
	}
}