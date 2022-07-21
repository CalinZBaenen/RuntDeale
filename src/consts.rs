use bevy::prelude::*;





/// The program's title. RUNTDEALE.
pub const PROGRAM_NAME:&str = "RuntDeale";

pub const WINDOW_HEIGHT:f32 = 600.;
pub const WINDOW_WIDTH:f32 = 800.;

/// Constructs a new generic description of the window.
pub(crate) fn get_window_description() -> WindowDescriptor {
	WindowDescriptor {
		cursor_visible: false,
		
		height: WINDOW_HEIGHT,
		width: WINDOW_WIDTH,
		
		title: PROGRAM_NAME.to_string(),
		
		..Default::default()
	}
}