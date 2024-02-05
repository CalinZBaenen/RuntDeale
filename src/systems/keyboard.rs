use crate::components::{
	general::Follow, entity::Direction,
	player::Player
};
use crate::params::{EntityWithSpritesheetQuery, CameraQuery};

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
	let mut r#move = Vec2::new(0., 0.);
	
	for &key in keys.get_pressed() {
		match key {
			KeyCode::Right | KeyCode::D => { r#move.x += 1.; }
			KeyCode::Down  | KeyCode::S => { r#move.y -= 1.; }
			KeyCode::Left  | KeyCode::A => { r#move.x -= 1.; }
			KeyCode::Up    | KeyCode::W => { r#move.y += 1.; }
			_ => {}
		}
	}
	
	if r#move.x != 0. && r#move.y != 0. { r#move = r#move.normalize_or_zero(); }
	if keys.pressed(KeyCode::ControlLeft) { r#move *= 2.; }
	
	let (mut nx, mut ny) = (0., 0.);
	for mut pq in &mut players {
		let mut attrs = pq.general;
		
		let dir = Direction::from_strongest(r#move.x, r#move.y).unwrap_or(**attrs.direction.as_ref().unwrap());
		(nx, ny) = attrs.bounds.shift(r#move.x, r#move.y);
		
		attrs.transform.translation.x = nx;
		attrs.transform.translation.y = ny;
		*attrs.direction.unwrap()     = dir;
		
		pq.ta_sprite.index = 3*(dir as usize);
	};
	
	for mut camera in &mut cameras {
		if !camera.is_primary { continue; }
		
		let follow = camera.follow.unwrap_or(&Follow::Both);
		let translation = Vec2::new(
			if follow.horizontal() { nx } else { 0. },
			if follow.vertical() { ny } else { 0. }
		);
		
		camera.transform.translation = translation.extend(camera.transform.translation.z);
	}
}