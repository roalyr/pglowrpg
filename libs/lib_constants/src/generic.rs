//▒▒▒▒▒▒▒▒▒▒▒▒ GENERIC ▒▒▒▒▒▒▒▒▒▒▒▒▒
pub const ONE_USIZE: usize = 1;
pub const ONE_U16: u16 = 1;
pub const ONE_U32: u32 = 1;
pub const ONE_F32: f32 = 1.0;

pub const ZERO_USIZE: usize = 0;
pub const ZERO_U8: u8 = 0;
pub const ZERO_U16: u16 = 0;
pub const ZERO_U32: u32 = 0;
pub const ZERO_F32: f32 = 0.0;

pub const VAL_255_F32: f32 = 255.0;
pub const VAL_127_F32: f32 = 127.0;

//▒▒▒▒▒▒▒▒▒▒▒▒ IDs ▒▒▒▒▒▒▒▒▒▒▒▒▒
// IDs that are written into BitLayer map.
pub const ID_MAP_NO_U32: u32 = 0;
pub const ID_MAP_MIN_U32: u32 = 1;

// IDs that are written into tables.
pub const UID_MIN_U32: u32 = 1;
pub const UID_MIN_U16: u16 = 1;

//▒▒▒▒▒▒▒▒▒▒▒▒ PATHFINDING ▒▒▒▒▒▒▒▒▒▒▒▒▒
// This values just has to be large enough, it will be used for 1st
// iteration and later on it will be adjusted on the 2nd run.
//pub const PATHFINDING_HEUR_INIT: usize = 1_000_000;

// This value is multiplied on estimated heuristic value in 2nd run,
// increasing or decreasing it. Lesser value = more precise pathfinding.
// Should be around 0.5 ... 2.0.
//pub const PATHFINDING_HEUR_FACTOR: usize = 0.5;

// This value should be around 5...50-ish in order for pathfinfing to be
// fast. It is the distance between the nodes between which A* will
// perform pathfinding, basically a key points to lead long paths.
//pub const PATHFINDING_SEGMENT_LENGTH: usize = 15;
