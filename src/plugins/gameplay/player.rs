use crate::components::{plane::Direction, marker::Player};
use crate::query::{PrimaryCameraQuery, PlayerQuery};

use bevy::ecs::{system::{Query, Res}, query::Without};
use bevy::input::{keyboard::KeyCode, Input};
use bevy::math::Vec2;





/// Handles the controls when the player should be in a `Roaming` state,
/// like on the map or in the battlebox.
pub(crate) fn sys_handle_freeroaming_controls(
	mut camera_query:Query<PrimaryCameraQuery, Without<Player>>,
	mut player_query:Query<PlayerQuery>,
	keys:Res<Input<KeyCode>>
) {
	let mut movement = Vec2::new(0., 0.);
	
	for &key in keys.get_pressed() {
		match key {
			KeyCode::Right | KeyCode::D => { movement.x += 1.; }
			KeyCode::Down  | KeyCode::S => { movement.y -= 1.; }
			KeyCode::Left  | KeyCode::A => { movement.x -= 1.; }
			KeyCode::Up    | KeyCode::W => { movement.y += 1.; }
			_ => {}
		}
	}
	
	movement = movement.normalize_or_zero();
	if keys.pressed(KeyCode::ControlLeft) { movement *= 2.; }
	
	let (mut nx, mut ny) = (0., 0.);
	for player in &mut player_query {
		let spatial_config = player.actor.spatial_config;
		
		// Get references to the components.
		let mut direction = spatial_config.direction.unwrap();
		let mut transform = spatial_config.transform;
		let     dir       = Direction::from_strongest(movement.x, movement.y).unwrap_or(*direction);
		
		// Shift the bounds of the player.
		(nx, ny) = spatial_config.bounds.unwrap().shift(movement.x, movement.y);
		
		// Set the transformation.
		transform.translation.x = nx;
		transform.translation.y = ny;
		
		// Change the direction 
		*direction = dir;
		if let Some(mut sprite) = player.actor.texture.ta_sprite { sprite.index = 3*(dir as usize); }
	};
	
	for camera in &mut camera_query {
		let camera = camera.camera;
		
		// Make the camera follow the player.
		let follow = camera.spatial_config.follow.unwrap();
		let translation = Vec2::new(
			if follow.horizontal() { nx } else { 0. },
			if follow.vertical() { ny } else { 0. }
		);
		
		// Move the camera.
		let mut transform     = camera.spatial_config.transform;
		transform.translation = translation.extend(	transform.translation.z);
	}
}