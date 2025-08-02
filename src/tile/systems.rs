use crate::tile::{Position, Tile, TileSprite, TileText};
use bevy::{
    color::palettes::css::{BLACK, LIGHT_GRAY, RED, YELLOW},
    prelude::*,
};

pub fn render_tiles(
    mut tile_sprite_query: Query<(&Tile, &mut Sprite), (With<TileSprite>, Changed<Tile>)>,
    mut tile_text_query: Query<(&Position, &mut Visibility), With<TileText>>,
    tile_data_query: Query<(&Position, &Tile), With<TileSprite>>,
) {
    for (tile, mut sprite) in tile_sprite_query.iter_mut() {
        if tile.is_revealed {
            sprite.color = if tile.is_mine {
                RED.into()
            } else {
                BLACK.into()
            };
        } else if tile.is_flagged {
            sprite.color = YELLOW.into();
        } else {
            sprite.color = LIGHT_GRAY.into();
        }
    }

    for (text_pos, mut visibility) in tile_text_query.iter_mut() {
        if let Some((_, tile)) = tile_data_query.iter().find(|(pos, _)| **pos == *text_pos) {
            *visibility = if tile.is_revealed && !tile.is_mine {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        }
    }
}
