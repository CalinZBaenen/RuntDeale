use crate::params::{EntityWithSpritesheetQueryItem, EntityWithSpritesheetQuery};
use crate::components::{entity::Direction, player::Player};

use bevy::ecs::{
	system::{Query, Res},
	query::With
};
use bevy::input::{keyboard::KeyCode, Input};





pub(crate) fn sys_handle_freeroaming_controls(
	mut players:Query<EntityWithSpritesheetQuery, With<Player>>,
	keys:Res<Input<KeyCode>>
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
	
	players.for_each_mut(move |mut data:EntityWithSpritesheetQueryItem<'_>| {
		let mut attrs = data.general;
		
		if mx > 0. && my > 0. {
			mx *= 0.7;
			my *= 0.7;
		}
		
		let dir = Direction::from_strongest(mx, my).unwrap_or(**attrs.direction.as_ref().unwrap());
		let (nx, ny) = attrs.bounds.shift(mx, my);
		
		attrs.transform.translation.x = nx;
		attrs.transform.translation.y = ny;
		*attrs.direction.unwrap()     = dir;
		
		data.ta_sprite.index = 3*(dir as usize);
	});
}