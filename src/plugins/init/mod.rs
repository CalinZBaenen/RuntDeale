pub(crate) mod load;

use crate::plugins::init::load::initsys_load_crcucial_textures;
use crate::components::{marker::Primary, plane::Follow};
use crate::resources::{BattleSS, Tilesets};
use crate::{InitSystem, Gamestate};
use crate::consts;

use bevy::window::{
	WindowResizeConstraints, WindowResolution,
	MonitorSelection, WindowPosition,
	WindowPlugin, Window
};
use bevy::core_pipeline::{
	clear_color::ClearColor,
	core_2d::Camera2dBundle
};
use bevy::render::{texture::ImagePlugin, color::Color, view::Msaa};
use bevy::ecs::schedule::{IntoSystemConfigs, apply_deferred};
use bevy::app::{PluginGroup, Startup, Plugin, App};
use bevy::ecs::system::Commands;
use bevy::DefaultPlugins;





pub(crate) struct InitGamePlugin;

impl Plugin for InitGamePlugin {
	fn build(&self, app:&mut App) {
		// Initialize and insert resources.
		app.init_resource::<BattleSS>();
		app.init_resource::<Tilesets>();
		
		app.insert_resource(ClearColor(Color::BLACK));
		app.insert_resource(Msaa::Off);
		
		
		// Add state(s).
		app.add_state::<Gamestate>();
		
		
		// Configure and add the default plugins.
		let defplugins = DefaultPlugins.set(WindowPlugin {
			primary_window: Some(Window {
				resize_constraints:WindowResizeConstraints {
					max_height:consts::program::DEFAULT_HEIGHT*1.25,
					min_height:consts::program::DEFAULT_HEIGHT,
					max_width:consts::program::DEFAULT_WIDTH*1.75,
					min_width:consts::program::DEFAULT_WIDTH
				},
				resolution:WindowResolution::new(
					consts::program::DEFAULT_WIDTH,
					consts::program::DEFAULT_HEIGHT
				),
				position:WindowPosition::Centered(MonitorSelection::Current),
				title:format!("{} ({})", consts::program::PROGRAM_NAME, consts::program::PROGRAM_VERSION),
				..Default::default()
			}),
			..Default::default()
		}).set(ImagePlugin::default_nearest());
		app.add_plugins(defplugins);
		
		
		// Add the essential systems.  (Ordered by schedule.)
		app.add_systems(Startup, (
			initsys_load_crcucial_textures.in_set(InitSystem),
			initsys_spawn_camera.in_set(InitSystem),
			apply_deferred
		).chain());
	}
}





/// Spawns the primary camera.
pub(crate) fn initsys_spawn_camera(mut commands:Commands) {
	commands.spawn(Camera2dBundle::default()).insert((Primary, Follow::Horizontal));
}