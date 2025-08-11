use crate::{
    board::BoardSettings,
    game::{components::OverlayText, events::*, resources::*, state::GameState},
    tile::{Position, Tile},
};
use bevy::{prelude::*, text::TextBounds};

pub fn handle_tile_revealed(
    settings: Res<BoardSettings>,
    mut game_stats: ResMut<GameStats>,
    mut tile_query: Query<(&Position, &mut Tile)>,
    mut game_over_events: EventWriter<GameOverEvent>,
    mut tile_revealed_events: EventReader<TileRevealedEvent>,
) {
    for event in tile_revealed_events.read() {
        if let Some((_, mut tile)) = tile_query
            .iter_mut()
            .find(|(pos, _)| **pos == event.position)
        {
            if tile.is_revealed || tile.is_flagged {
                continue;
            }

            tile.reveal();
            game_stats.tiles_revealed += 1;

            if tile.is_mine {
                game_over_events.write(GameOverEvent);
                return;
            }

            if tile.adjacent_mines == 0 {
                reveal_adjacent_tiles(event.position, &mut tile_query, &settings);
            }
        }
    }
}
pub fn handle_chord_tile(
    settings: Res<BoardSettings>,
    mut chord_events: EventReader<ChordEvent>,
    mut tile_query: Query<(&Position, &mut Tile)>,
    mut tile_revealed_events: EventWriter<TileRevealedEvent>,
) {
    for event in chord_events.read() {
        for dy in -1..=1i8 {
            for dx in -1..=1i8 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let new_x = event.position.x as i8 + dx;
                let new_y = event.position.y as i8 + dy;

                if new_x >= 0
                    && new_y >= 0
                    && new_x < settings.width as i8
                    && new_y < settings.height as i8
                {
                    let adjacent_pos = Position {
                        x: new_x as u8,
                        y: new_y as u8,
                    };

                    if let Some((_, mut adjacent_tile)) =
                        tile_query.iter_mut().find(|(pos, _)| **pos == adjacent_pos)
                    {
                        tile_revealed_events.write(TileRevealedEvent {
                            position: adjacent_pos,
                        });
                    }
                }
            }
        }
    }
}

fn reveal_adjacent_tiles(
    position: Position,
    tile_query: &mut Query<(&Position, &mut Tile)>,
    settings: &BoardSettings,
) {
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

                if let Some((_, mut adjacent_tile)) =
                    tile_query.iter_mut().find(|(pos, _)| **pos == adjacent_pos)
                {
                    if !adjacent_tile.is_revealed
                        && !adjacent_tile.is_flagged
                        && !adjacent_tile.is_mine
                    {
                        adjacent_tile.reveal();

                        if adjacent_tile.adjacent_mines == 0 {
                            reveal_adjacent_tiles(adjacent_pos, tile_query, settings);
                        }
                    }
                }
            }
        }
    }
}

pub fn handle_tile_flagged(
    mut tile_flagged_events: EventReader<TileFlaggedEvent>,
    mut tile_query: Query<(&Position, &mut Tile)>,
    mut game_stats: ResMut<GameStats>,
) {
    for event in tile_flagged_events.read() {
        if let Some((_, mut tile)) = tile_query
            .iter_mut()
            .find(|(pos, _)| **pos == event.position)
        {
            if tile.is_revealed {
                continue;
            }

            tile.toggle_flag();

            if tile.is_flagged {
                game_stats.mines_remaining -= 1;
            } else {
                game_stats.mines_remaining += 1;
            }
        }
    }
}

pub fn check_win_condition(
    tile_query: Query<&Tile>,
    settings: Res<BoardSettings>,
    mut game_won_events: EventWriter<GameWonEvent>,
) {
    let total_tiles = settings.width as usize * settings.height as usize;
    let revealed_tiles = tile_query.iter().filter(|tile| tile.is_revealed).count() as usize;
    let mine_count = settings.mine_count as u16;

    if revealed_tiles == total_tiles - mine_count as usize {
        game_won_events.write(GameWonEvent);
    }
}

pub fn handle_game_over(
    mut game_over_events: EventReader<GameOverEvent>,
    mut next_state: ResMut<NextState<GameState>>,
    mut tile_query: Query<&mut Tile>,
    mut commands: Commands,
) {
    for _ in game_over_events.read() {
        next_state.set(GameState::GameOver);

        for mut tile in tile_query.iter_mut() {
            if tile.is_mine {
                tile.reveal();
            }
        }

        show_overlay_text(&mut commands, "Game Over! Press R to restart".to_string());
    }
}

pub fn handle_game_won(
    mut game_won_events: EventReader<GameWonEvent>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
) {
    for _ in game_won_events.read() {
        next_state.set(GameState::Won);

        show_overlay_text(&mut commands, "You Won! Press R to restart".to_string());
    }
}

fn show_overlay_text(commands: &mut Commands, text: String) {
    let box_size: Vec2 = Vec2::new(320.0, 25.0);
    commands
        .spawn((
            Sprite::from_color(Color::srgba(0., 0., 0., 0.7), box_size),
            OverlayText,
        ))
        .with_children(|builder| {
            builder.spawn((
                Text2d::new(text),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextBounds::from(box_size),
            ));
        });
}

fn remove_overlay_screen(
    commands: &mut Commands,
    overlay_entities: Query<Entity, With<OverlayText>>,
) {
    for entity in overlay_entities.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn handle_new_game(
    mut new_game_events: EventReader<NewGameEvent>,
    mut next_state: ResMut<NextState<GameState>>,
    mut game_stats: ResMut<GameStats>,
    mut commands: Commands,
    overlay_entities: Query<Entity, With<OverlayText>>,
) {
    for _ in new_game_events.read() {
        *game_stats = GameStats::default();
        next_state.set(GameState::Playing);

        remove_overlay_screen(&mut commands, overlay_entities);
    }
}

pub fn reset_game_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut new_game_events: EventWriter<NewGameEvent>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        new_game_events.write(NewGameEvent);
    }
}
