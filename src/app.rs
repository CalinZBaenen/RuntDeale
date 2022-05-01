use std::option::Option;
	use Option::Some;

use piston::input::{RenderArgs, UpdateArgs};
use glutin_window::GlutinWindow as Window;
use opengl_graphics::GlGraphics;
use piston::event_loop::Events;
use graphics::math::Scalar;
use piston::{Event, Input};

use crate::rd_objects::{sprites, actors};
use crate::rd_utils::colors;
use crate::rd_events;







/**
* "Main class".
* Place to store metadata about the application.
*/
pub struct App {
	pub player:actors::Entity,
	pub gl:GlGraphics
}

/**
* Implementation for the app.
* Contains logic that updates and renders the program.
*/
impl App {
	pub fn render(&mut self, args:&RenderArgs) -> () {
		// NEW RENDERING LOGIC.
	}
	
	
	
	
	
	pub fn update(&mut self, args:&UpdateArgs) -> () {
		// UPDATE LOGIC.
	}
	
	
	
	
	
	pub fn init(&mut self, win:&mut Window, mut events:Events) -> Events {
		while let Some(e) = events.next(win) {
			match e {
				Event::Custom(_, _, _) => {}
				
				Event::Input(Input::Button(b), _) => {
					rd_events::buttons::handle_button(self, &b);
				}
				
				Event::Loop(loopdata) => {
					rd_events::gameloop::handle_loop(self, &loopdata);
				}
				
				_ => {}
			}
		}
		
		events
	}
}