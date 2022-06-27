pub use crate::components::{AutoMoving, GameState, Player, JUMP_SPEED};
use crate::components::{GameOverUIInputTimer, ResetGameEvent};
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

pub fn handle_game_over_input(
    mut reset_game_event: EventWriter<ResetGameEvent>,
    mut game_state: ResMut<State<GameState>>,
    mut keyboard_input: ResMut<Input<KeyCode>>,
    mut mouse_button_input: ResMut<Input<MouseButton>>,
    touches: ResMut<Touches>,
    time: Res<Time>,
    mut query: Query<(Entity, &mut GameOverUIInputTimer)>,
) {
    let (_, mut timer) = query.single_mut();
    timer.0.tick(time.delta());

    if !timer.0.finished() {
        return;
    }

    if mouse_button_input.just_pressed(MouseButton::Left)
        || keyboard_input.just_pressed(KeyCode::Space)
    {
        reset_game_event.send(ResetGameEvent);
        game_state.set(GameState::Paused).unwrap();
        mouse_button_input.reset(MouseButton::Left);
        keyboard_input.reset(KeyCode::Space);
    }

    for _touch in touches.iter_just_pressed() {
        reset_game_event.send(ResetGameEvent);
        game_state.set(GameState::Paused).unwrap();
        break;
    }
}
pub fn handle_input_system(
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
