use crate::{
    board::BoardSettings,
    game::GameStats,
    tile::{Position, Tile, TileBundle, TileSprite, TileText, TileTextBundle},
    utils::{calculate_tile_x, calculate_tile_y},
};
use bevy::{color::palettes::css::BLACK, prelude::*};
use rand::{prelude::*, rng};

#[derive(Component)]
pub struct BoardBackground;

pub fn setup_board(
    mut commands: Commands,
    settings: Res<BoardSettings>,
    mut game_stats: ResMut<GameStats>,
) {
    render_board_bg(&mut commands, &settings);
    spawn_tiles(&mut commands, &settings);
    *game_stats = GameStats::default();
}

pub fn reset_board(
    mut commands: Commands,
    settings: Res<BoardSettings>,
    mut game_stats: ResMut<GameStats>,
    tile_entities: Query<Entity, Or<(With<TileSprite>, With<TileText>)>>,
    bg_entities: Query<Entity, With<BoardBackground>>,
) {
    for entity in tile_entities.iter() {
        commands.entity(entity).despawn();
    }
    for entity in bg_entities.iter() {
        commands.entity(entity).despawn();
    }

    render_board_bg(&mut commands, &settings);
    spawn_tiles(&mut commands, &settings);
    *game_stats = GameStats::default();
}

fn render_board_bg(commands: &mut Commands, settings: &BoardSettings) {
    let total_width = settings.width as f32 * settings.tile_size;
    let total_height = settings.height as f32 * settings.tile_size;

    commands.spawn((
        Sprite::from_color(BLACK, Vec2::new(total_width, total_height)),
        Transform::from_translation(Vec3::new(0.0, 0.0, -2.0)),
        BoardBackground,
    ));
}

fn spawn_tiles(commands: &mut Commands, settings: &BoardSettings) {
    let mines = generate_mines(settings);

    for y in 0..settings.height {
        for x in 0..settings.width {
            let position = Position { x, y };
            let index = (y * settings.width + x) as usize;
            let is_mine = mines[index];

            let adjacent_mines = if is_mine {
                0
            } else {
                count_adjacent_mines(&mines, settings.width, settings.height, x, y)
            };

            let tile = Tile {
                is_mine,
                is_revealed: false,
                is_flagged: false,
                adjacent_mines,
            };

            let tile_x = calculate_tile_x(position.x, settings.width, settings.tile_size);
            let tile_y = calculate_tile_y(position.y, settings.height, settings.tile_size);

            commands.spawn(TileBundle {
                sprite: Sprite::from_color(
                    Color::srgb(0.7, 0.7, 0.7),
                    Vec2::new(settings.tile_size - 1.0, settings.tile_size - 1.0),
                ),
                transform: Transform::from_translation(Vec3::new(tile_x, tile_y, 0.0)),
                tile: tile.clone(),
                position,
                tile_sprite: TileSprite,
            });

            if !is_mine && adjacent_mines > 0 {
                let text_color = get_color_from_mine_count(adjacent_mines);

                commands.spawn(TileTextBundle {
                    text: Text2d::new(adjacent_mines.to_string()),
                    text_layout: TextLayout::new_with_justify(JustifyText::Center),
                    transform: Transform::from_translation(Vec3::new(tile_x, tile_y, 1.0)),
                    tile_text: TileText,
                    text_color: TextColor(text_color),
                    position,
                });
            }
        }
    }
}

fn get_color_from_mine_count(mines: u8) -> Color {
    match mines {
        1 => Color::srgb(0.0, 0.0, 1.0), // Blue
        2 => Color::srgb(0.0, 0.5, 0.0), // Green
        3 => Color::srgb(1.0, 0.0, 0.0), // Red
        4 => Color::srgb(0.5, 0.0, 0.5), // Purple
        5 => Color::srgb(0.5, 0.0, 0.0), // Dark Red
        6 => Color::srgb(0.0, 0.5, 0.5), // Teal
        7 => Color::srgb(0.0, 0.0, 0.0), // Black
        8 => Color::srgb(0.5, 0.5, 0.5), // Gray
        _ => Color::srgb(0.0, 0.0, 0.0),
    }
}

fn generate_mines(settings: &BoardSettings) -> Vec<bool> {
    let mut rng = rng();
    let total_tiles = (settings.width * settings.height) as usize;
    let mut mines = vec![false; total_tiles];

    let mut mines_placed = 0;
    while mines_placed < settings.mine_count {
        let index = rng.random_range(0..total_tiles);
        if !mines[index] {
            mines[index] = true;
            mines_placed += 1;
        }
    }

    mines
}

fn count_adjacent_mines(mines: &[bool], width: u8, height: u8, x: u8, y: u8) -> u8 {
    let mut count = 0;
    for dy in -1..=1i8 {
        for dx in -1..=1i8 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let temp_x = x as i8 + dx;
            let temp_y = y as i8 + dy;
            if temp_x >= 0 && temp_y >= 0 && temp_x < width as i8 && temp_y < height as i8 {
                let index = (temp_y as u8 * width + temp_x as u8) as usize;
                if mines[index] {
                    count += 1;
                }
            }
        }
    }
    count
}
