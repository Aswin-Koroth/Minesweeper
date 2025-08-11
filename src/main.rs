#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod board;
mod config;
mod game;
mod input;
mod tile;
mod utils;

use bevy::{prelude::*, window::WindowResolution};
use board::BoardPlugin;
use config::{WINDOW_HEIGHT, WINDOW_WIDTH};
use game::{GamePlugin, GameState};
use input::InputPlugin;
use tile::TilePlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Minesweeper v1.0.1".into(),
                    resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT)
                        .with_scale_factor_override(1.0),
                    ..default()
                }),
                ..default()
            }),
        )
        .init_state::<GameState>()
        .add_plugins((GamePlugin, BoardPlugin, TilePlugin, InputPlugin))
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
