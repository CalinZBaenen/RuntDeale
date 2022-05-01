use piston::{
	ControllerButton, ButtonArgs,
	Button, Key
};
	use Button::Controller;
	use Button::Keyboard;

use crate::App;







// Reroute button data based on type.
pub fn handle_button(app:&mut App, args:&ButtonArgs) -> () {
	match args.button {
		Controller(b) => { handle_ctrlrBtn(app, b); }
		Keyboard(k)   => { handle_kbKey(app, k); }
		
		_ => {}
	}
}





pub fn handle_ctrlrBtn(_app:&mut App, _b:ControllerButton) -> () {
	// Implement controller controls here, maybe.
}





pub fn handle_kbKey(app:&mut App, k:Key) -> () {
	match k {
		Key::A     => { /* KEY 'A' EVENT. */ }
		Key::D     => { /* KEY 'D' EVENT. */ }
		Key::W     => { /* KEY 'W' EVENT. */ }
		Key::S     => { /* KEY 'S' EVENT. */ }
		Key::Space => { /* SPACE KEY EVENT. */ }
		
		_ => {}
	}
}