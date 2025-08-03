pub mod components;
pub mod events;
pub mod resources;
pub mod state;
pub mod systems;

use bevy::prelude::*;
pub use events::*;
pub use resources::*;
pub use state::GameState;
use systems::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TileRevealedEvent>()
            .add_event::<TileFlaggedEvent>()
            .add_event::<GameOverEvent>()
            .add_event::<GameWonEvent>()
            .add_event::<NewGameEvent>()
            .init_resource::<GameStats>()
            .add_systems(
                Update,
                (
                    reset_game_input,
                    handle_new_game,
                    (
                        handle_tile_revealed,
                        handle_tile_flagged,
                        check_win_condition,
                        handle_game_over,
                        handle_game_won,
                    )
                        .run_if(in_state(GameState::Playing)),
                ),
            );
    }
}
