use bevy::prelude::*;
use crate::tile::Position;

#[derive(Event)]
pub struct TileRevealedEvent {
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