use crate::components::{
	plane::{Direction, Bounds},
	actor::PlayerStats,
	texture::Texture,
	marker::Player
};

use bevy::ecs::bundle::Bundle;





/// A bundle representing player entities.
#[derive(Bundle)]
pub struct PlayerBundle {
	/// The direction the player is facing.
	pub        direction:Direction,
	/// Player marker.
	pub(crate) _player:Player,
	/// Incorporated attributes from the `ActorBundle` bundle.
	pub        actor:ActorBundle,
	/// The player's statistics.
	pub        stats:PlayerStats
}

impl PlayerBundle {
	pub fn new(direction:Direction, stats:PlayerStats, actor:ActorBundle) -> Self {
		Self {
			direction,
			_player:Player,
			actor,
			stats
		}
	}
}



/// A bundle representing actor entities.
#[derive(Bundle)]
pub struct ActorBundle {
	/// The actor's texture.
	pub texture:Texture,
	/// The bounds this actor lies within.
	pub bounds:Bounds
}