use crate::components::texture::Texture;

use bevy::ecs::{
	system::{Commands, Query},
	entity::Entity,
	query::Added
};
use bevy::app::{Plugin, First, App};





pub(crate) struct TexturePlugin;

impl Plugin for TexturePlugin {
	fn build(&self, app:&mut App) {
		app.add_systems(First, sys_unwrap_texture);
	}
}





/// Unwraps `Texture` components and inserts the respective
/// `SpriteSheetBundle` and `SpriteBundle` components.
pub(crate) fn sys_unwrap_texture(
	mut textured_items:Query<(Entity, &mut Texture), Added<Texture>>,
	mut commands:Commands
) {
	for textured in &mut textured_items {
		let     texture = textured.1.to_owned();
		let mut entity = commands.entity(textured.0);
		
		match texture {
			Texture::Spritesheet(ss) => {
				entity.insert(ss);
				entity.remove::<Texture>();
			}
			Texture::Sprite(s) => {
				entity.insert(s);
				entity.remove::<Texture>();
			}
			_ => {}
		}
	}
}