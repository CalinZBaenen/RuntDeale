mod components;
mod geometry;
mod plugins;
mod sysres;
mod consts;
	
use bevy::app::App;





fn main() {
	let _ = App::new().add_plugins(plugins::init::InitGamePlugin).run();
}