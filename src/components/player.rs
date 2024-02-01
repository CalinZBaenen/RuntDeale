use crate::components::entity::{EntityBundle, Direction};

use bevy::ecs::component::Component;
use bevy::ecs::bundle::Bundle;





#[derive(Bundle)]
pub struct PlayerBundle {
	pub(crate) attributes:EntityBundle,
	pub(crate) direction:Direction,
	pub(crate) player:Player
}



#[derive(Component, Clone, Copy)]
pub struct Player;