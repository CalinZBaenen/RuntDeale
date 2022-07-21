use bevy::prelude::*;

mod components;
//mod plugins;
mod consts;





pub fn main() {
	App::new()
		.insert_resource(ClearColor( Color::rgb(0., 0., 0.) ))
		.insert_resource(consts::get_window_description())
		.add_startup_system(init_game)
		.add_plugins(DefaultPlugins)
		.run();
}

/// Stuff for initializing stuff such as the camera.  
/// I dunno.
pub(crate) fn init_game(mut cmds:Commands) {
	cmds.spawn_bundle(OrthographicCameraBundle::new_2d())
		.insert(components::MainCamera);
}