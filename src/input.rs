pub use crate::components::{GameState, Player, JUMP_SPEED};
use bevy::prelude::*;

pub fn handle_menu_input(
    mut game_state: ResMut<State<GameState>>,
    keyboard_input: Res<Input<KeyCode>>,
    mouse_button_input: Res<Input<MouseButton>>,
    touches: Res<Touches>,
) {
    if mouse_button_input.just_pressed(MouseButton::Left) {
        game_state.set(GameState::Running).unwrap();
    }

    if keyboard_input.just_pressed(KeyCode::Space) {
        game_state.set(GameState::Running).unwrap();
    }

    for _touch in touches.iter_just_pressed() {
        game_state.set(GameState::Running).unwrap();
    }
}
pub fn handle_input_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mouse_button_input: Res<Input<MouseButton>>,
    touches: Res<Touches>,
    mut query: Query<(&mut Player)>,
) {
    let mut player = query.single_mut();

    if mouse_button_input.just_pressed(MouseButton::Left) {
        player.movement_speed = JUMP_SPEED;
    }

    if keyboard_input.just_pressed(KeyCode::Space) {
        player.movement_speed = JUMP_SPEED;
    }

    for _touch in touches.iter_just_pressed() {
        player.movement_speed = JUMP_SPEED;
    }
}
