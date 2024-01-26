use crate::consts;
use crate::sysres;

use bevy::window::{
	WindowResizeConstraints, WindowResolution,
	MonitorSelection, WindowPosition,
	WindowPlugin, Window
};
use bevy::app::{PluginGroup, Update, Startup, Plugin, App};
use bevy::ecs::schedule::IntoSystemConfigs;
use bevy::ecs::schedule::States;
use bevy::DefaultPlugins;

use std::fmt;





pub(crate) struct InitGamePlugin;

impl Plugin for InitGamePlugin {
	fn build(&self, app:&mut App) {
		app.init_resource::<sysres::BattleSS>();
		app.init_resource::<sysres::PlayerSS>();
		app.init_resource::<sysres::Tilesets>();
		
		app.add_systems(Startup, (
			sysres::texture::load_essential_game_textures,
			sysres::texture::spawn_camera,
			sysres::draw::draw_player
		).chain());
		
		app.add_state::<Gamestate>();
		
		app.add_plugins(
			DefaultPlugins
				.set(WindowPlugin {
					primary_window: Some(Window {
						resize_constraints:WindowResizeConstraints {
							max_height:consts::DEFAULT_HEIGHT*1.25,
							min_height:consts::DEFAULT_HEIGHT,
							max_width:consts::DEFAULT_WIDTH*1.75,
							min_width:consts::DEFAULT_WIDTH
						},
						resolution:WindowResolution::new(consts::DEFAULT_WIDTH, consts::DEFAULT_HEIGHT),
						position:WindowPosition::Centered(MonitorSelection::Current),
						title:format!("{} ({})", consts::PROGRAM_NAME, consts::PROGRAM_VERSION),
						..Default::default()
					}),
					..Default::default()
				})
		);
	}
}



#[derive(PartialEq, States, Clone, Debug, Copy, Hash, Eq)]
pub(crate) enum Gamestate {
	Dialogue,
	Roaming,
	Battle,
	Menu
}

impl Default for Gamestate {
	fn default() -> Self { Self::Menu }
}

impl fmt::Display for Gamestate {
	fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", match self {
			Self::Dialogue => "dialogue",
			Self::Roaming  => "roaming mode",
			Self::Battle   => "battle mode",
			Self::Menu     => "in menu"
		})
	}
}