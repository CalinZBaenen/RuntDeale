use super::posmas::Location;

use bevy::ecs::component::Component;
use bevy::ecs::bundle::Bundle;





#[derive(Bundle)]
pub struct PlayerBundle {
	location:Location,
	player:Player
}



#[derive(Component, Clone, Copy)]
pub struct Player;