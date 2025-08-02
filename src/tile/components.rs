use bevy::prelude::*;

#[derive(Component, Clone, Debug)]
pub struct Tile {
    pub is_mine: bool,
    pub is_flagged: bool,
    pub is_revealed: bool,
    pub adjacent_mines: u8,
}

impl Tile {
    pub fn toggle_flag(&mut self) {
        self.is_flagged = !self.is_flagged;
    }
}

#[derive(Component, Copy, Clone, Debug, PartialEq)]
pub struct Position {
    pub x: u8,
    pub y: u8,
}

#[derive(Component, Debug)]
pub struct TileSprite;

#[derive(Component, Debug)]
pub struct TileText;

#[derive(Bundle)]
pub struct TileBundle {
    pub tile: Tile,
    pub sprite: Sprite,
    pub position: Position,
    pub transform: Transform,
    pub tile_sprite: TileSprite,
}

#[derive(Bundle)]
pub struct TileTextBundle {
    pub text: Text2d,
    pub position: Position,
    pub tile_text: TileText,
    pub transform: Transform,
    pub text_color: TextColor,
    pub text_layout: TextLayout,
}
