pub mod resources;
pub mod systems;

use crate::game::{events::NewGameEvent};
use bevy::prelude::*;
pub use resources::*;
use systems::*;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BoardSettings>()
            .add_systems(Startup, setup_board)
            .add_systems(Update, reset_board.run_if(on_event::<NewGameEvent>));
    }
}
