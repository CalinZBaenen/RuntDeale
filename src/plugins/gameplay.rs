use crate::systems::{
	game::{
		keyboard::sys_handle_freeroaming_controls,
		spawn::sys_spawn_player
	},
	init::initsys_load_crcucial_textures
};
use crate::Gamestate;

use bevy::ecs::{
	schedule::{
		common_conditions::in_state, IntoSystemConfigs,
		NextState, OnEnter
	},
	system::ResMut
};
use bevy::app::{Plugin, Update, App};





pub(crate) struct GameplayPlugin;

impl Plugin for GameplayPlugin {
	fn build(&self, app:&mut App) {
		app.add_systems(OnEnter(Gamestate::EnteringMap), (
			sys_spawn_player,
			|mut state:ResMut<NextState<Gamestate>>| { state.set(Gamestate::Roaming) }
		));
		
		app.add_systems(
			Update,
			sys_handle_freeroaming_controls.run_if( in_state(Gamestate::Roaming) )
				.after(initsys_load_crcucial_textures)
		);
	}
}