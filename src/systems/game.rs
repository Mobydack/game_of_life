use bevy::prelude::*;

use crate::states::*;

pub fn game_states_controller(
    state: Res<State<GamePauseStates>>,
    kb_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GamePauseStates>>,
) {
    if kb_input.just_pressed(KeyCode::KeyP) {
        match state.get() {
            GamePauseStates::Paused => next_state.set(GamePauseStates::Running),
            GamePauseStates::Running => next_state.set(GamePauseStates::Paused),
        }
    }
}
