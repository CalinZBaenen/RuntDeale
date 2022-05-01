mod app;
mod rd_events;
mod rd_objects;
mod rd_utils;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::window::WindowSettings;

use app::App;
use rd_objects::actors::Entity; // Main application interface.

fn main() {
    let opengl_ver = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("RuntDeale", [500, 500])
        .graphics_api(opengl_ver)
        .exit_on_esc(true)
        .build()
        .unwrap();
    let mut app = App {
        player: Entity::create(String::from("Frisk"), 1.0, 1.0, 1.0, 1.0),
        gl: GlGraphics::new(opengl_ver),
    };
    let mut events = Events::new(EventSettings::new());

    app.init(&mut window, events);
}
