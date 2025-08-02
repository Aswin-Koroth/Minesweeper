use crate::config::*;
use bevy::prelude::*;

#[derive(Resource, Debug)]
pub struct BoardSettings {
    pub width: u8,
    pub height: u8,
    pub tile_size: f32,
    pub mine_count: u8,
}

impl Default for BoardSettings {
    fn default() -> Self {
        Self {
            width: BOARD_WIDTH,
            height: BOARD_HEIGHT,
            tile_size: TILE_SIZE,
            mine_count: MINE_COUNT,
        }
    }
}
