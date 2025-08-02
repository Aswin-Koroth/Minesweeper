pub mod components;
pub mod systems;

use bevy::prelude::*;
pub use components::*;
use systems::*;

pub struct TilePlugin;

impl Plugin for TilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, render_tiles);
    }
}
