use crate::geometry::*;

use bevy::ecs::component::Component;





#[derive(PartialEq, Component)]
struct Collidable {
	boundingbox:Rectangle<f32>,
	solid:bool,
}