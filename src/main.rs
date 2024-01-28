pub        mod components;
pub        mod geometry;
pub(crate) mod plugins;
pub        mod consts;
pub(crate) mod sysres;

use bevy::app::App;





fn main() {
	let _ = App::new().add_plugins(plugins::init::InitGamePlugin).run();
}