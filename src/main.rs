pub        mod components;
pub        mod resources;
pub        mod geometry;
pub(crate) mod plugins;
pub        mod systems;
pub        mod consts;
pub        mod params;

use bevy::app::App;





fn main() {
	let _ = App::new().add_plugins((
		plugins::controls::GameControlsPlugin,
		plugins::init::InitGamePlugin
	)).run();
}