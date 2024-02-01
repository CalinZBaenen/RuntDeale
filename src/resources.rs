use bevy::ecs::system::Resource;
use bevy::sprite::TextureAtlas;
use bevy::asset::Handle;

use std::collections::HashMap;





#[derive(PartialEq, Resource, Default, Clone, Debug, Eq)]
pub(crate) struct BattleSS(pub(crate) HashMap<&'static str, Handle<TextureAtlas>>);



#[derive(PartialEq, Resource, Clone, Debug, Eq)]
pub(crate) struct PlayerSS(pub(crate) Handle<TextureAtlas>);



#[derive(PartialEq, Resource, Default, Clone, Debug, Eq)]
pub(crate) struct Tilesets(pub(crate) HashMap<&'static str, Handle<TextureAtlas>>);