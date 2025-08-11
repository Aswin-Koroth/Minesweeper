use crate::tile::Position;
use bevy::prelude::*;

#[derive(Event)]
pub struct TileRevealedEvent {
    pub position: Position,
}

#[derive(Event)]
pub struct ChordEvent {
    pub position: Position,
}

#[derive(Event)]
pub struct TileFlaggedEvent {
    pub position: Position,
}

#[derive(Event)]
pub struct GameOverEvent;

#[derive(Event)]
pub struct GameWonEvent;

#[derive(Event)]
pub struct NewGameEvent;