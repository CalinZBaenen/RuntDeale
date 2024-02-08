use crate::components::{
	actor::{EntityBundle, PlayerBundle, Player},
	plane::Bounds
};
use crate::consts::config::SCALE_FACTOR;
use crate::resources::PlayerSS;

use bevy::sprite::{TextureAtlasSprite, SpriteSheetBundle, Anchor};
use bevy::transform::components::Transform;
use bevy::ecs::system::{Commands, Res};
use bevy::math::Vec3;





pub fn sys_spawn_player(mut commands:Commands, player_texture:Res<PlayerSS>) {
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
			sprites:ssb.clone(),
			bounds: Bounds {
				physical_bounds:(0., 0., 20., 29.).into(),
				..Default::default()
			}
		},
		direction: Default::default(),
		player:Player
	});
	
	commands.spawn(EntityBundle {
		sprites:ssb.clone(),
		bounds: Bounds {
			physical_bounds:(0., 0., 20., 29.).into(),
			..Default::default()
		}
	});
}