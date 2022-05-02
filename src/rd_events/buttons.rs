use piston::{Button, ButtonArgs, ControllerButton, Key};
use Button::Controller;
use Button::Keyboard;

use crate::App;

// Reroute button data based on type.
pub fn handle_button(app: &mut App, args: &ButtonArgs) {
    match args.button {
        Controller(b) => {
            handle_ctrlr_btn(app, b);
        }
        Keyboard(k) => {
            handle_kb_key(app, k);
        }

        _ => {}
    }
}

pub fn handle_ctrlr_btn(_app: &mut App, _b: ControllerButton) {
    // Implement controller controls here, maybe.
}

pub fn handle_kb_key(app: &mut App, k: Key) {
    match k {
        Key::A => { /* KEY 'A' EVENT. */ }
        Key::D => { /* KEY 'D' EVENT. */ }
        Key::W => { /* KEY 'W' EVENT. */ }
        Key::S => { /* KEY 'S' EVENT. */ }
        Key::Space => { /* SPACE KEY EVENT. */ }

        _ => {}
    }
}
