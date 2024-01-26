use super::PlayerSS;

use bevy::prelude::*;





pub(crate) fn draw_player(mut commands:Commands, player_texture:Res<PlayerSS>) {
	commands.spawn(SpriteSheetBundle {
		texture_atlas: player_texture.0.clone(),
		transform: Transform {
			scale:Vec3::new(2.75, 2.75, 1.0),
			..Default::default()
		},
		sprite: TextureAtlasSprite::new(0),
		..Default::default()
	});
}