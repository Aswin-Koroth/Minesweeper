use crate::{
    board::BoardSettings,
    game::events::*,
    tile::{Position, Tile, TileSprite},
    utils::{calculate_tile_x, calculate_tile_y},
};
use bevy::prelude::*;

pub fn handle_mouse_input(
    windows: Query<&Window>,
    settings: Res<BoardSettings>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    camera: Single<(&Camera, &GlobalTransform)>,
    mut tile_query: Query<(&Position, &Tile), With<TileSprite>>,
    mut tile_flagged_events: EventWriter<TileFlaggedEvent>,
    mut tile_revealed_events: EventWriter<TileRevealedEvent>,
    mut chord_events: EventWriter<ChordEvent>,
) {
    let left_click = mouse_input.just_pressed(MouseButton::Left);
    let right_click = mouse_input.just_pressed(MouseButton::Right);

    if !left_click && !right_click {
        return;
    }

    let (camera, camera_transform) = *camera;
    let Ok(window) = windows.single() else {
        return;
    };

    if let Some(cursor_position) = window.cursor_position() {
        if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_position) {
            if let Some((clicked_position, tile)) =
                get_clicked_tile(world_pos, &settings, &tile_query)
            {
                if left_click {
                    if tile.is_revealed {
                        let adjacent_flags_count =
                            get_adjacent_flags_count(clicked_position, &mut tile_query, &settings);

                        if adjacent_flags_count == tile.adjacent_mines {
                            chord_events.write(ChordEvent {
                                position: clicked_position,
                            });
                        }
                    }
                    tile_revealed_events.write(TileRevealedEvent {
                        position: clicked_position,
                    });
                } else if right_click {
                    tile_flagged_events.write(TileFlaggedEvent {
                        position: clicked_position,
                    });
                }
            }
        }
    }
}

fn get_clicked_tile(
    world_pos: Vec2,
    settings: &BoardSettings,
    tile_query: &Query<(&Position, &Tile), With<TileSprite>>,
) -> Option<(Position, Tile)> {
    let tile_size = settings.tile_size;
    for (pos, tile) in tile_query.iter() {
        let tile_world_pos = Vec2::new(
            calculate_tile_x(pos.x, settings.width, tile_size),
            calculate_tile_y(pos.y, settings.height, tile_size),
        );
        if world_pos.distance(tile_world_pos) < tile_size / 2. {
            return Some((*pos, tile.clone()));
        }
    }
    None
}

fn get_adjacent_flags_count(
    position: Position,
    tile_query: &mut Query<(&Position, &Tile), With<TileSprite>>,
    settings: &BoardSettings,
) -> u8 {
    let mut flags = 0;

    for dy in -1..=1i8 {
        for dx in -1..=1i8 {
            if dx == 0 && dy == 0 {
                continue;
            }

            let new_x = position.x as i8 + dx;
            let new_y = position.y as i8 + dy;

            if new_x >= 0
                && new_y >= 0
                && new_x < settings.width as i8
                && new_y < settings.height as i8
            {
                let adjacent_pos = Position {
                    x: new_x as u8,
                    y: new_y as u8,
                };

                if let Some((_, adjacent_tile)) =
                    tile_query.iter_mut().find(|(pos, _)| **pos == adjacent_pos)
                {
                    if adjacent_tile.is_flagged {
                        flags += 1;
                    }
                }
            }
        }
    }

    flags
}
