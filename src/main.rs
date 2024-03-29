pub        mod components;
pub        mod resources;
pub        mod entities;
pub(crate) mod plugins;
pub        mod consts;
pub        mod query;

use bevy_embedded_assets::{EmbeddedAssetPlugin, PluginMode as EmbedMode};
use bevy::ecs::schedule::{SystemSet, States};
use bevy::app::App;

use std::fmt;





#[derive(PartialEq, SystemSet, Clone, Debug, Copy, Hash, Eq)]
pub struct InitSystem;



/// The current state of the game.
#[derive(PartialEq, States, Clone, Debug, Copy, Hash, Eq)]
pub enum Gamestate {
	/// The player is about to enter the 'map' portion of the game. (`EnteringMap` -> `Roaming`)
	EnteringMap,
	/// The player is onscreen and can move about freely.
	Roaming,
	/// A menu is present.
	Menu
}

impl Default for Gamestate {
	fn default() -> Self { Self::EnteringMap }
}

impl fmt::Display for Gamestate {
	fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", match self {
			Self::EnteringMap => "*loading map*",
			Self::Roaming     => "roaming mode",
			Self::Menu        => "in menu"
		})
	}
}





fn main() {
	let mut app = App::new();
	
	app.add_plugins((
		EmbeddedAssetPlugin {mode: EmbedMode::ReplaceDefault},
		plugins::gameplay::GameplayPlugin,
		plugins::movement::MovementPlugin,
		plugins::texture::TexturePlugin,
		plugins::init::InitGamePlugin
	));
	
	app.run();
}