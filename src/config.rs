pub const BOARD_HEIGHT: u8 = 10;
pub const BOARD_WIDTH: u8 = 10;
pub const MINE_COUNT: u8 = 10;
pub const TILE_SIZE: f32 = 32.0;

const WINDOW_PADDING: f32 = 20.0;
pub const WINDOW_WIDTH: f32 = (BOARD_WIDTH as f32 * TILE_SIZE) + WINDOW_PADDING;
pub const WINDOW_HEIGHT: f32 = (BOARD_HEIGHT as f32 * TILE_SIZE) + WINDOW_PADDING;
