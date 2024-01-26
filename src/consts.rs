/// Program metadata.
pub mod program {
	/// Program version number.
	pub const PROGRAM_VERSION:&'static str = env!("CARGO_PKG_VERSION");
	/// The default height for the program.
	pub const DEFAULT_HEIGHT:f32 = 600f32;
	/// The default width for the program.
	pub const DEFAULT_WIDTH:f32 = 800f32;
	/// Program name  :  RUNTDEALE.
	pub const PROGRAM_NAME:&'static str = "RuntDeale";
}



pub(crate) mod texpath {
	pub const TS_RAINVILLE:&'static str = "textures/tiles/rainville.png";
	
	pub const SS_BATTLE_BUTTONS:&'static str = "textures/battle/buttons.png";
	pub const SS_CHLOE:&'static str = "textures/characters/chloe.png";
	pub const SS_SOULS:&'static str = "textures/battle/souls.png";
}



/// Game configuration constants.
pub(crate) mod config {
	/// The maximum amount of bullets that can be present in one scene.
	pub const MAX_BULLET_COUNT:u32 = 50;
	/// The player-character's name.
	pub const PLAYER_NAME:&'static str = "Chloe";
	/// The standard amount of padding around tilesets.
	pub const SS_PADDING:f32 = 1.0;
	/// The standard size of a tile's width and height.
	pub const TILE_SIZE:f32 = 10.0;
}