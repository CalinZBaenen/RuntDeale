use piston::Loop;

use crate::App;

// Reroute loop data based on type.
pub fn handle_loop(app: &mut App, l: &Loop) {
    match l {
        Loop::AfterRender(_) => {}

        Loop::Render(args) => {
            app.render(args);
        }
        Loop::Update(args) => {
            app.update(args);
        }

        Loop::Idle(_) => {}
    }
}
