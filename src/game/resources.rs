use bevy::prelude::*;

#[derive(Resource, Debug)]
pub struct GameStats {
    pub mines_remaining: i16,
    pub tiles_revealed: u16,
}

impl Default for GameStats {
    fn default() -> Self {
        Self {
            mines_remaining: crate::config::MINE_COUNT as i16,
            tiles_revealed: 0,
        }
    }
}
