use crate::components::{entity::Bounds, player::Player};
use crate::sysres::params::GeneralEntityParams;

use bevy::ecs::{query::With, system::Res, world::Mut};
use bevy::input::{keyboard::KeyCode, Input};
use bevy::transform::components::Transform;





pub(crate) fn sys_handle_freeroaming_controls(
	keys:Res<Input<KeyCode>>,
	mut query:GeneralEntityParams<With<Player>>
) {
	let (mut mx, mut my) = (0., 0.);
	
	for &key in keys.get_pressed() {
		match key {
			KeyCode::Right | KeyCode::D => { mx += 1.; }
			KeyCode::Down | KeyCode::S => { my -= 1.; }
			KeyCode::Left | KeyCode::A => { mx -= 1.; }
			KeyCode::Up | KeyCode::W => { my += 1.; }
			_ => {}
		}
	}
	if keys.pressed(KeyCode::ControlLeft) {
		mx *= 2.;
		my *= 2.;
	}
	
	query.enq.for_each_mut(move |(mut bounds, mut transform):(Mut<Bounds>, Mut<Transform>)| {
		if mx > 0. && my > 0. {
			mx *= 0.7;
			my *= 0.7;
		}
		
		let (nx, ny) = bounds.shift(mx, my);
		transform.translation.x = nx;
		transform.translation.y = ny;
	});
}