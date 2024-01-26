// Program metadata.
/// Program version number.
pub const PROGRAM_VERSION:&'static str = env!("CARGO_PKG_VERSION");
/// The default height for the program.
pub const DEFAULT_HEIGHT:f32 = 600f32;
/// The default width for the program.
pub const DEFAULT_WIDTH:f32 = 800f32;
/// Program name  :  RUNTDEALE.
pub const PROGRAM_NAME:&'static str = "RuntDeale";



// Game configuration.
/// The maximum amount of bullets that can be present in one scene.
pub(crate) const MAX_BULLET_COUNT:u32 = 50;
/// The standard amount of padding around tilesets.
pub(crate) const STD_SS_PADDING:f32 = 1.0;
/// The standard size of a tile's width and height.
pub(crate) const STD_TILE_SIZE:f32 = 10.0;
/// The player-character's name.
pub(crate) const PLAYER_NAME:&'static str = "Chloe";