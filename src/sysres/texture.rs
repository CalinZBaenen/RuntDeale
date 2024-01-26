use super::{BattleSS, PlayerSS, Tilesets};
use crate::consts;

use bevy::prelude::*;





pub(crate) fn load_essential_game_textures(
	mut battle_textures:ResMut<BattleSS>,
	mut texture_atlases:ResMut<Assets<TextureAtlas>>,
	mut commands:Commands,
	mut tilesets:ResMut<Tilesets>,
	    assets:ResMut<AssetServer>
) {
	// Player.
	commands.insert_resource(PlayerSS(texture_atlases.add(TextureAtlas::from_grid(
		assets.load("textures/characters/chloe.png"),
		Vec2::new(20.0, 29.0),
		3, 4,
		Some(Vec2::splat(consts::STD_SS_PADDING)),
		Some(Vec2::splat(consts::STD_SS_PADDING))
	))));
	
	// World stuff.
	tilesets.0.insert("rainville", texture_atlases.add(TextureAtlas::from_grid(
		assets.load("textures/tiles/rainville.png"),
		Vec2::splat(consts::STD_TILE_SIZE),
		14, 3,
		Some(Vec2::splat(consts::STD_SS_PADDING)),
		Some(Vec2::splat(consts::STD_SS_PADDING))
	)));
	
	// Battle stuff.
	battle_textures.0.insert("buttons", texture_atlases.add(TextureAtlas::from_grid(
		assets.load("textures/battle/buttons.png"),
		Vec2::new(200.0, 100.0),
		4, 2,
		None, None
	)));
	battle_textures.0.insert("souls", texture_atlases.add(TextureAtlas::from_grid(
		assets.load("textures/battle/souls.png"),
		Vec2::splat(9.0),
		7, 2,
		None, None
	)));
}



pub(crate) fn spawn_camera(mut commands:Commands) {
	// let camera = Camera2dBundle::default();
	commands.spawn(Camera2dBundle::default());
}