use crate::{
    board::BoardSettings,
    game::events::*,
    tile::{Position, TileSprite},
    utils::{calculate_tile_x, calculate_tile_y},
};
use bevy::prelude::*;

pub fn handle_mouse_input(
    windows: Query<&Window>,
    settings: Res<BoardSettings>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    camera: Single<(&Camera, &GlobalTransform)>,
    tile_query: Query<&Position, With<TileSprite>>,
    mut tile_flagged_events: EventWriter<TileFlaggedEvent>,
    mut tile_revealed_events: EventWriter<TileRevealedEvent>,
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
            if let Some(clicked_position) =
                get_clicked_tile_position(world_pos, &settings, &tile_query)
            {
                if left_click {
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

fn get_clicked_tile_position(
    world_pos: Vec2,
    settings: &BoardSettings,
    tile_query: &Query<&Position, With<TileSprite>>,
) -> Option<Position> {
    let tile_size = settings.tile_size;
    for pos in tile_query.iter() {
        let tile_world_pos = Vec2::new(
            calculate_tile_x(pos.x, settings.width, tile_size),
            calculate_tile_y(pos.y, settings.height, tile_size),
        );
        if world_pos.distance(tile_world_pos) < tile_size / 2. {
            return Some(*pos);
        }
    }
    None
}
