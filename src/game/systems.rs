use crate::{
    board::BoardSettings,
    game::{events::*, resources::*, state::GameState},
    tile::{Position, Tile},
};
use bevy::prelude::*;

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

            tile.is_revealed = true;
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
                        adjacent_tile.is_revealed = true;

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
    mut game_won_events: EventWriter<GameWonEvent>,
    settings: Res<BoardSettings>,
) {
    let total_tiles = (settings.width * settings.height) as u16;
    let revealed_tiles = tile_query.iter().filter(|tile| tile.is_revealed).count() as u16;
    let mine_count = settings.mine_count as u16;

    if revealed_tiles == total_tiles - mine_count {
        game_won_events.write(GameWonEvent);
    }
}

pub fn handle_game_over(
    mut game_over_events: EventReader<GameOverEvent>,
    mut next_state: ResMut<NextState<GameState>>,
    mut tile_query: Query<&mut Tile>,
) {
    for _ in game_over_events.read() {
        println!("Game Over! Press R to restart");
        next_state.set(GameState::GameOver);

        for mut tile in tile_query.iter_mut() {
            if tile.is_mine {
                tile.is_revealed = true;
            }
        }
    }
}

pub fn handle_game_won(
    mut game_won_events: EventReader<GameWonEvent>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for _ in game_won_events.read() {
        println!("You Won! Press R to restart");
        next_state.set(GameState::Won);
    }
}

pub fn handle_new_game(
    mut new_game_events: EventReader<NewGameEvent>,
    mut next_state: ResMut<NextState<GameState>>,
    mut game_stats: ResMut<GameStats>,
) {
    for _ in new_game_events.read() {
        info!("New Game");
        *game_stats = GameStats::default();
        next_state.set(GameState::Playing);
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
