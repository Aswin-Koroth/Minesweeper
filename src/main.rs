mod board;
mod controls;
mod tile;

use bevy::{prelude::*, window::WindowResolution};
use board::{BoardSettings, reset_board, setup_board};
use controls::{handle_flag_click, handle_tile_click};
use tile::render_tiles;

const BOARD_HEIGHT: u8 = 12;
const BOARD_WIDTH: u8 = 12;
const MINE_COUNT: u8 = 10;
const TILE_SIZE: f32 = 32.0;

const WINDOW_WIDTH: f32 = BOARD_WIDTH as f32 * TILE_SIZE;
const WINDOW_HEIGHT: f32 = BOARD_HEIGHT as f32 * TILE_SIZE;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Minesweeper".into(),
                    resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT)
                        .with_scale_factor_override(1.0),
                    ..default()
                }),
                ..default()
            }),
        )
        .insert_resource(BoardSettings {
            height: BOARD_HEIGHT,
            width: BOARD_WIDTH,
            tile_size: TILE_SIZE,
            mine_count: MINE_COUNT,
        })
        .add_systems(Startup, (setup_camera, setup_board))
        .add_systems(
            Update,
            (
                handle_tile_click,
                handle_flag_click,
                render_tiles,
                reset_board,
            ),
        )
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}
