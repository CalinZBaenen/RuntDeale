pub        mod components;
pub        mod resources;
pub        mod geometry;
pub(crate) mod plugins;
pub        mod systems;
pub        mod consts;
pub        mod params;

use bevy::ecs::schedule::States;
use bevy::app::App;

use std::fmt;





#[derive(PartialEq, States, Clone, Debug, Copy, Hash, Eq)]
pub enum Gamestate {
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





fn main() {
	let _ = App::new().add_plugins((
		plugins::controls::GameControlsPlugin,
		plugins::init::InitGamePlugin
	)).run();
}