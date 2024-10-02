use bevy::prelude::*;

#[derive(Debug, Clone, Default, Copy, Eq, PartialEq, States, Hash)]
pub enum GamePauseStates {
    #[default]
    Running,
    Paused,
}
