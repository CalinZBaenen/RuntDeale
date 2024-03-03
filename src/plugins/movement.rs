use crate::components::{marker::{Primary, Player}, plane::Direction};
use crate::query::{PrimaryCameraQuery, PlayerQuery};
use crate::Gamestate;

use bevy::ecs::{
	schedule::{common_conditions::in_state, IntoSystemConfigs},
	query::{Changed, Without, With},
	system::{Query, Res},
};
use bevy::input::{keyboard::KeyCode, Input};
use bevy::transform::components::Transform;
use bevy::app::{Plugin, Update, App};
use bevy::math::{Vec3, Vec2};





/// Plugin used to handle movement.
pub(crate) struct MovementPlugin;

impl Plugin for MovementPlugin {
	fn build(&self, app:&mut App) {
		app.add_systems(Update, (
			sys_handle_freeroaming_controls,
			sys_move_camera_with_player
		).chain().run_if( in_state(Gamestate::Roaming) ));
	}
}



/// Structure to handle movement.
#[allow(dead_code)]
pub(crate) enum Movement {
	Three(Vec3),
	Two(Vec2)
}





/// Handles the controls when the player should be in a `Roaming` state,
/// like on the map or in the battlebox.
pub(crate) fn sys_handle_freeroaming_controls(
	mut player_query:Query<PlayerQuery, With<Primary>>,
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
	
	for player in &mut player_query {
		let spatial_config = player.actor.spatial_config;
		
		// Get references to the components.
		let mut direction = spatial_config.direction.unwrap();
		let mut transform = spatial_config.transform;
		let     dir       = Direction::from_strongest(movement.x, movement.y).unwrap_or(*direction);
		
		// Shift the bounds of the player.
		let (nx, ny) = spatial_config.bounds.unwrap().shift(movement.x, movement.y);
		
		// Set the transformation.
		move_entity(&mut transform, Movement::Two(Vec2::new(nx, ny)));
		
		// Change the direction 
		*direction = dir;
		if let Some(mut sprite) = player.actor.texture.ta_sprite { sprite.index = 3*(dir as usize); }
	};
}



fn sys_move_camera_with_player(
	mut camera_query:Query<PrimaryCameraQuery, Without<Player>>,
	player_query:Query<PlayerQuery, (Changed<Transform>, With<Primary>)>
) {
	let camera_item = camera_query.get_single_mut().unwrap();
	let player_item = player_query.get_single();
	
	if let Ok(player_item) = player_item {
		let follow = camera_item.camera.spatial_config.follow.unwrap();
		
		let mut camera_transform = camera_item.camera.spatial_config.transform;
		let     player_transform = player_item.actor.spatial_config.transform;
		
		let translation = Vec2::from( follow.follow_or(
			(player_transform.translation.x, player_transform.translation.y),
			(camera_transform.translation.x, camera_transform.translation.y)
		) );
		
		move_entity(&mut camera_transform, Movement::Two(translation));
	}
}



pub(crate) fn move_entity(transform:&mut Transform, location:Movement) {
	match location {
		Movement::Three(v3) => transform.translation = v3,
		Movement::Two(v2)   => transform.translation = v2.extend(transform.translation.z)
	}
}