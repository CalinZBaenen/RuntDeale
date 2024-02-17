pub(crate) mod spawn;

use crate::plugins::gameplay::spawn::sys_spawn_player;
use crate::Gamestate;

use bevy::ecs::{
	schedule::{NextState, OnEnter},
	system::ResMut
};
use bevy::app::{Plugin, App};





pub(crate) struct GameplayPlugin;

impl Plugin for GameplayPlugin {
	fn build(&self, app:&mut App) {
		app.add_systems(OnEnter(Gamestate::EnteringMap), (
			sys_spawn_player,
			|mut state:ResMut<NextState<Gamestate>>| { state.set(Gamestate::Roaming) }
		));
	}
}