use crate::components::entity::EntityBundle;

use bevy::ecs::component::Component;
use bevy::ecs::bundle::Bundle;





#[derive(Bundle)]
pub struct PlayerBundle {
	pub(crate) attributes:EntityBundle,
	pub(crate) player:Player
}



#[derive(Component, Clone, Copy)]
pub struct Player;