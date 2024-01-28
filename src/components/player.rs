use super::general::Location;

use bevy::ecs::component::Component;
use bevy::sprite::SpriteSheetBundle;
use bevy::ecs::bundle::Bundle;





#[derive(Bundle)]
pub struct PlayerBundle {
	location:Location,
	sprites:SpriteSheetBundle,
	player:Player
}



#[derive(Component, Clone, Copy)]
pub struct Player;