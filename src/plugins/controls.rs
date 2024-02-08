use crate::systems::game::keyboard::sys_handle_freeroaming_controls;

use bevy::app::{Plugin, Update, App};





pub(crate) struct GameControlsPlugin;

impl Plugin for GameControlsPlugin {
	fn build(&self, app:&mut App) {
		app.add_systems(Update, sys_handle_freeroaming_controls);
	}
}