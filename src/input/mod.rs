pub mod systems;

use crate::{game::state::GameState, input::systems::handle_mouse_input};
use bevy::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_mouse_input).run_if(in_state(GameState::Playing)),
        );
    }
}
