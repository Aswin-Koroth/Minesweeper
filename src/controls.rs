use crate::{
    board::BoardSettings,
    tile::{Position, Tile},
};
use bevy::prelude::*;

pub fn handle_tile_click(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera: Single<(&Camera, &GlobalTransform)>,
    settings: Res<BoardSettings>,
    mut tile_query: Query<(&Position, &mut Tile)>,
) {
    if !mouse_button_input.just_pressed(MouseButton::Left) {
        return;
    }

    let (camera, camera_transform) = *camera;
    let Ok(window) = windows.single() else {
        return;
    };
    if let Some(cursor_position) = window.cursor_position() {
        if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_position) {
            for (pos, mut tile) in tile_query.iter_mut() {
                let tile_size = settings.tile_size;
                let tile_world_pos = Vec2::new(
                    (pos.x as f32 * tile_size) - ((tile_size / 2.) * (settings.width - 1) as f32),
                    (pos.y as f32 * tile_size) - ((tile_size / 2.) * (settings.height - 1) as f32),
                );

                if world_pos.x >= tile_world_pos.x - (tile_size / 2.)
                    && world_pos.x < tile_world_pos.x + (tile_size / 2.)
                    && world_pos.y >= tile_world_pos.y - (tile_size / 2.)
                    && world_pos.y < tile_world_pos.y + (tile_size / 2.)
                {
                    if !tile.is_flagged {
                        tile.is_revealed = true;
                        if tile.is_mine {
                            println!("Game Over!");
                        }
                    }
                    break;
                }
            }
        }
    }
}

pub fn handle_flag_click(mouse_button_input: Res<ButtonInput<MouseButton>>) {
    if !mouse_button_input.just_pressed(MouseButton::Right) {
        return;
    }
    println!("flagged")
}
