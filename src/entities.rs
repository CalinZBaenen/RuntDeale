use crate::components::{
	plane::{Direction, Bounds},
	actor::PlayerConfig,
	texture::Texture,
	marker::Player
};

use bevy::ecs::bundle::Bundle;





#[derive(Bundle)]
pub struct PlayerBundle {
	pub attributes:ActorBundle,
	pub direction:Direction,
	pub config:PlayerConfig,
	pub player:Player
}



#[derive(Bundle)]
pub struct ActorBundle {
	pub(crate) texture:Texture,
	pub(crate) bounds:Bounds
}