use bevy::{math::vec3, prelude::*, sprite::collide_aabb::collide, window::PresentMode};
use rand::prelude::*;
pub mod input;
use input::{handle_game_over_input, handle_input_system, handle_menu_input};
pub mod components;
use components::*;
pub mod setup;
use setup::*;

fn player_movement_system(
    time: Res<Time>,
    gravity: Res<Gravity>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut Player,
        &mut Transform,
        &TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    let (mut player, mut transform, sprite, texture_atlas_handle) = query.single_mut();
    let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
    let texture_height = texture_atlas.size.y;

    let new_speed = player.movement_speed + gravity.0;
    player.movement_speed = new_speed.clamp(-1000.0, 10000.0);

    let new_player_pos = transform.translation.y
        + vec3(
            PLAYER_POS_X,
            player.movement_speed * time.delta_seconds(),
            2.0,
        );
    transform.translation = new_player_pos.clamp(
        vec3(PLAYER_POS_X, FLOOR_POS + 90.0, 2.0),
        vec3(PLAYER_POS_X, SCREEN.y / 2.0, 2.0),
    );

    if player.movement_speed > 0.0 {
        player.angle = 30.0;
    } else {
        player.angle = (player.angle - 180.0 * time.delta_seconds()).clamp(-90.0, 45.0);
    }

    transform.rotation = Quat::from_rotation_z(f32::to_radians(player.angle));
}

fn animate_sprite_system(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}

fn auto_move_system(mut query: Query<(&AutoMoving, &mut Transform, Option<&mut Countable>)>) {
    let mut rng = thread_rng();

    for (auto_moving, mut transform, countable) in query.iter_mut() {
        transform.translation.x -= AUTO_MOVE_SPEED;

        // if out of screen -> move to other side
        if transform.translation.x + auto_moving.width / 2.0 < -SCREEN.x / 2.0 {
            if let Some(mut countable) = countable {
                countable.0 = true;
            }
            transform.translation.x =
                SCREEN.x / 2.0 + auto_moving.width / 2.0 + auto_moving.displacement;
            transform.translation.y =
                auto_moving.initial.y + auto_moving.randomness.y * rng.gen_range(-1.0..1.0);
        }
    }
}

fn animate_world(mut query: Query<(&SpeedAnimated, &mut Transform)>) {
    let iter = query.iter_mut();
    let total: f32 = iter.len() as f32;
    for (speed_animated, mut transform) in iter {
        transform.translation.x -= AUTO_MOVE_SPEED;

        if transform.translation.x + speed_animated.width / 2.0 < -SCREEN.x / 2.0 {
            transform.translation.x = transform.translation.x + (speed_animated.width * total);
        }
    }
}

fn point_count_system(
    mut scoreboard: ResMut<Scoreboard>,
    mut countable_query: Query<(&mut Countable, &Transform)>,
    player_query: Query<(&Player, &Transform)>,
) {
    let (_, player_transform) = player_query.single();

    for (mut countable, transform) in countable_query.iter_mut() {
        if !countable.0 {
            continue;
        }

        if transform.translation.x < player_transform.translation.x {
            countable.0 = false;
            scoreboard.score += 1;
        }
    }
}

fn update_score_text(scoreboard: Res<Scoreboard>, mut query: Query<(&ScoreText, &mut Text)>) {
    for (_, mut text) in query.iter_mut() {
        text.sections.get_mut(0).unwrap().value = scoreboard.score.to_string();
    }
}

fn collision_system(
    mut game_state: ResMut<State<GameState>>,
    collider_query: Query<(&Collider, &GlobalTransform)>,
    blocker_query: Query<(&Blocker, &GlobalTransform)>,
) {
    for (_, c_transf) in collider_query.iter() {
        for (blocker, b_transf) in blocker_query.iter() {
            let collision = collide(
                c_transf.translation,
                PLAYER,
                b_transf.translation,
                blocker.0,
            );
            match collision {
                Some(_collision) => {
                    println!(
                        "c_pos: {}, c_size: {}",
                        c_transf.translation.to_string(),
                        PLAYER
                    );
                    println!(
                        "b_pos: {}, b_size: {}",
                        b_transf.translation.to_string(),
                        PIPE
                    );
                    game_state.set(GameState::GameOver).unwrap();
                    break;
                }
                None => {
                    continue;
                }
            }
        }
    }
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Flappy bird in Rust".to_string(),
            width: SCREEN.x,
            height: SCREEN.y,
            present_mode: PresentMode::Fifo,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .insert_resource(Scoreboard { score: 0 })
        .insert_resource(Gravity(SCALED_GRAVITY))
        .add_state(GameState::Paused)
        .add_startup_system(setup_player)
        .add_startup_system(setup_floor)
        .add_startup_system(setup_pipes)
        .add_startup_system(setup_font)
        .add_startup_system(setup_game_over_ui)
        .add_event::<ResetGameEvent>()
        .add_system_set(
            SystemSet::on_update(GameState::Paused)
                .with_system(handle_menu_input)
                .with_system(animate_world)
                .with_system(animate_sprite_system),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Running)
                .with_system(auto_move_system)
                .with_system(animate_world)
                .with_system(collision_system)
                .with_system(point_count_system)
                .with_system(update_score_text)
                .with_system(handle_input_system)
                .with_system(animate_sprite_system)
                .with_system(player_movement_system),
        )
        .add_system_set(
            SystemSet::on_enter(GameState::GameOver).with_system(set_game_over_ui_visible),
        )
        .add_system_set(
            SystemSet::on_update(GameState::GameOver)
                .with_system(handle_game_over_input)
                .with_system(player_movement_system),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::GameOver)
                .with_system(game_over_cleanup)
                .with_system(set_game_over_ui_hidden),
        )
        .run();
}
