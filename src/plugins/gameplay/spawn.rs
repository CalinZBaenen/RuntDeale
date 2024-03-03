use crate::components::{
	plane::{Direction, Bounds},
	marker::{Primary, Player},
	actor::PlayerStats,
	texture::Texture,
};
use crate::entities::{PlayerBundle, ActorBundle};
use crate::consts::config::SCALE_FACTOR;
use crate::resources::PlayerSS;

use bevy::sprite::{TextureAtlasSprite, SpriteSheetBundle, Anchor};
use bevy::transform::components::Transform;
use bevy::ecs::system::{Commands, Res};
use bevy::math::Vec3;





/// Spawns the player.
pub fn sys_spawn_player(mut commands:Commands, player_texture:Res<PlayerSS>) {
	let ssb = SpriteSheetBundle {
		texture_atlas: player_texture.0.clone(),
		transform: Transform {
			scale: Vec3::new(SCALE_FACTOR, SCALE_FACTOR, 1.),
			..Default::default()
		},
		sprite: TextureAtlasSprite {
			anchor: Anchor::TopLeft,
			..Default::default()
		},
		..Default::default()
	};
	
	commands.spawn(PlayerBundle {
		direction:Direction::default(),
		_player:Player,
		actor: ActorBundle {
			texture: Texture::from(ssb.clone()),
			bounds: Bounds::flat(0., 0., 20., 29.)
		},
		stats: PlayerStats::default()
	}).insert(Primary);
	
	commands.spawn(ActorBundle {
		texture: Texture::from(ssb.clone()),
		bounds: Bounds::flat(0., 0., 20., 29.)
	});
}