use bevy::{
    color::palettes::css::{DIM_GRAY, RED, YELLOW},
    prelude::*,
};

#[derive(Component, Clone, Debug)]
pub struct Tile {
    pub is_mine: bool,
    pub is_flagged: bool,
    pub is_revealed: bool,
    pub adjacent_mines: u8,
}

#[derive(Component, Copy, Clone, Debug)]
pub struct Position {
    pub x: u8,
    pub y: u8,
}

pub fn render_tiles(mut tile_query: Query<(&Tile, &mut Sprite), Changed<Tile>>) {
    for (tile, mut sprite) in tile_query.iter_mut() {
        if tile.is_revealed {
            sprite.color = if tile.is_mine {
                RED.into()
            } else {
                Color::NONE
            };
        } else if tile.is_flagged {
            sprite.color = YELLOW.into();
        } else {
            sprite.color = DIM_GRAY.into();
        }
    }
}
