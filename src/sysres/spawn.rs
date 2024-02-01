use crate::components::{
	entity::{EntityBundle, Bounds},
	player::{PlayerBundle, Player},
	general::Primary
};
use crate::consts::config::SCALE_FACTOR;
use crate::sysres::PlayerSS;

use bevy::sprite::{TextureAtlasSprite, SpriteSheetBundle, Anchor};
use bevy::core_pipeline::core_2d::Camera2dBundle;
use bevy::transform::components::Transform;
use bevy::ecs::system::{Commands, Res};
use bevy::math::Vec3;





pub(crate) fn sys_spawn_camera(mut commands:Commands) {
	commands.spawn(Camera2dBundle::default()).insert(Primary);
}



pub(crate) fn sys_spawn_player(mut commands:Commands, player_texture:Res<PlayerSS>) {
	let ssb = SpriteSheetBundle {
		texture_atlas: player_texture.0.clone(),
		transform: Transform {
			scale:Vec3::new(SCALE_FACTOR, SCALE_FACTOR, 1.),
			..Default::default()
		},
		sprite: TextureAtlasSprite {
			anchor:Anchor::TopLeft,
			..Default::default()
		},
		..Default::default()
	};
	
	commands.spawn(PlayerBundle {
		attributes: EntityBundle {
			sprites:ssb,
			bounds: Bounds {
				physical_bounds:(0., 0., 20., 29.).into(),
				..Default::default()
			}
		},
		direction: Default::default(),
		player:Player
	});
}