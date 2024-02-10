use bevy::ecs::system::Resource;
use bevy::sprite::TextureAtlas;
use bevy::asset::Handle;

use std::collections::HashMap;





/// Resource for the spritesheets associated with battles.
#[derive(PartialEq, Resource, Default, Clone, Debug, Eq)]
pub struct BattleSS(pub(crate) HashMap<&'static str, Handle<TextureAtlas>>);



/// Resource for the player-character's spritesheet.
#[derive(PartialEq, Resource, Clone, Debug, Eq)]
pub struct PlayerSS(pub(crate) Handle<TextureAtlas>);



/// Resource for the spritesheets containing textures for tiles.
#[derive(PartialEq, Resource, Default, Clone, Debug, Eq)]
pub struct Tilesets(pub(crate) HashMap<&'static str, Handle<TextureAtlas>>);