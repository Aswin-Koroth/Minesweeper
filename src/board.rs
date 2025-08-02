use crate::tile::{Position, Tile};
use bevy::{color::palettes::css::BLACK, prelude::*};
use rand::{self, Rng, rng};

#[derive(Resource, Debug)]
pub struct BoardSettings {
    pub width: u8,
    pub height: u8,
    pub tile_size: f32,
    pub mine_count: u8,
}

fn setup_board_bg(commands: &mut Commands, settings: &Res<BoardSettings>) {
    let total_width = settings.width as f32 * settings.tile_size;
    let total_height = settings.height as f32 * settings.tile_size;

    let background_transform = Transform::from_translation(Vec3::new(0.0, 0.0, -2.0));

    commands.spawn((
        Sprite::from_color(BLACK, Vec2::new(total_width, total_height)),
        background_transform,
    ));
}

pub fn setup_board(mut commands: Commands, settings: Res<BoardSettings>) {
    let mut rng = rng();
    let mut mines = vec![false; (settings.width * settings.height) as usize];

    for _ in 0..settings.mine_count {
        let mut i;
        loop {
            i = rng.random_range(0..mines.len());
            if !mines[i] {
                mines[i] = true;
                break;
            }
        }
    }

    let tile_size = settings.tile_size;
    setup_board_bg(&mut commands, &settings);
    for y in 0..settings.height {
        for x in 0..settings.width {
            let index = (y * settings.width + x) as usize;
            let is_mine = mines[index];
            let pos = Position { x, y };

            let adjacent_mines =
                count_adjacent_mines(&mines, settings.width, settings.height, x, y);

            let tile = Tile {
                is_mine,
                is_revealed: false,
                is_flagged: false,
                adjacent_mines,
            };

            let tile_x =
                (pos.x as f32 * tile_size) - ((tile_size / 2.) * (settings.width - 1) as f32);
            let tile_y =
                (pos.y as f32 * tile_size) - ((tile_size / 2.) * (settings.height - 1) as f32);

            if !is_mine && adjacent_mines > 0 {
                commands.spawn((
                    Text2d::new(tile.adjacent_mines.to_string()),
                    TextLayout::new(JustifyText::Center, LineBreak::WordBoundary),
                    Transform::from_translation(Vec3::new(tile_x, tile_y, -1.0)),
                    tile.clone(),
                ));
            }

            commands.spawn((
                Sprite::from_color(Color::srgb(1., 0., 0.), Vec2::new(tile_size, tile_size)),
                Transform::from_translation(Vec3::new(tile_x, tile_y, 0.0)),
                tile.clone(),
                pos,
            ));
        }
    }
}

fn count_adjacent_mines(mines: &Vec<bool>, width: u8, height: u8, x: u8, y: u8) -> u8 {
    let mut count = 0;
    for dy in -1..=1 {
        for dx in -1..=1 {
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

pub fn reset_board(mut tile_query: Query<&mut Tile>, keyboard_input: Res<ButtonInput<KeyCode>>) {
    if !keyboard_input.just_pressed(KeyCode::KeyR) {
        return;
    }
    for mut tile in tile_query.iter_mut() {
        tile.is_revealed = false;
        tile.is_flagged = false;
    }
}
