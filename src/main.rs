use bevy::{
    math::{const_vec2, const_vec3, vec2, vec3},
    prelude::*,
    sprite::collide_aabb::collide,
    window::PresentMode,
};
use rand::prelude::*;
pub mod input;
use input::{handle_input_system, handle_menu_input};
pub mod components;
use components::*;

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
            0.0,
        );
    transform.translation = new_player_pos.clamp(
        vec3(PLAYER_POS_X, FLOOR_POS + 90.0, 0.0),
        vec3(PLAYER_POS_X, SCREEN_HEIGHT / 2.0, 0.0),
    );

    if player.movement_speed > 0.0 {
        player.angle = 45.0;
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

fn auto_move_system(
    game_state: Res<State<GameState>>,
    mut query: Query<(&AutoMoving, &mut Transform, Option<&mut Countable>)>,
) {
    if game_state.current().eq(&GameState::GameOver) {
        return;
    }

    let mut rng = thread_rng();

    for (auto_moving, mut transform, countable) in query.iter_mut() {
        transform.translation.x -= AUTO_MOVE_SPEED;

        // if out of screen -> move to other side
        if transform.translation.x + auto_moving.width / 2.0 < -SCREEN_WIDTH / 2.0 {
            if let Some(mut countable) = countable {
                countable.0 = true;
            }
            transform.translation.x =
                SCREEN_WIDTH / 2.0 + auto_moving.width / 2.0 + auto_moving.displacement;
            transform.translation.y =
                auto_moving.initial.y + auto_moving.randomness.y * rng.gen_range(-1.0..1.0);
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
    let (_, mut text) = query.single_mut();
    text.sections.get_mut(0).unwrap().value = scoreboard.score.to_string();
}

fn collision_system(
    mut game_state: ResMut<State<GameState>>,
    collider_query: Query<(&Collider, &Transform)>,
    blocker_query: Query<(&Blocker, &GlobalTransform)>,
) {
    for (_, c_transf) in collider_query.iter() {
        for (_, b_transf) in blocker_query.iter() {
            let collision = collide(
                c_transf.translation,
                PLAYER_DIM,
                b_transf.translation,
                PIPE_DIM,
            ); // hmmmm funker ikke
            match collision {
                Some(_collision) => {
                    game_state.set(GameState::GameOver).unwrap();
                }
                None => {
                    continue;
                }
            }
        }
    }
}

fn setup_font(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("flappy-font.ttf");
    let style = TextStyle {
        font,
        font_size: 100.0,
        color: Color::WHITE,
    };
    let alignment = TextAlignment {
        vertical: VerticalAlign::Center,
        horizontal: HorizontalAlign::Center,
    };

    commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section("0", style, alignment),
            transform: Transform::from_xyz(0.0, SCREEN_HEIGHT / 4.0, 1.0),
            ..default()
        })
        .insert(ScoreText);
}

fn setup_pipes(mut commands: Commands, asset_server: Res<AssetServer>) {
    let pipe_handle = asset_server.load("sprites/pipe-green.png");
    let mut rng = thread_rng();

    for n in 0..2 {
        let rand_num = if rng.gen_ratio(1, 2) {
            rng.gen_range(0.5..1.0)
        } else {
            rng.gen_range(-1.0..-0.5)
        };

        let parent = commands
            .spawn_bundle(TransformBundle {
                local: Transform {
                    translation: vec3(
                        PIPE_START_X + n as f32 * SPACE_BETWEEN_PIPES,
                        n as f32 * (PIPE_RANDOM_Y * rand_num),
                        0.0,
                    ),
                    ..Default::default()
                },
                ..default()
            })
            .insert(AutoMoving {
                width: PIPE_WIDTH * 2.0,
                displacement: SPACE_BETWEEN_PIPES / 2.0,
                randomness: vec3(0.0, PIPE_RANDOM_Y, 0.0),
                initial: vec3(0.0, 0.0, 0.0),
            })
            .insert(Countable(true))
            .id();
        let pipe_top = pipe_handle.clone();
        let pipe_bottom = pipe_top.clone();
        let space_between = SCREEN_HEIGHT / 3.5;
        let child_top = commands
            .spawn_bundle(SpriteBundle {
                texture: pipe_top,
                ..default()
            })
            .insert_bundle(TransformBundle {
                local: Transform {
                    translation: vec3(0.0, PIPE_HEIGHT + space_between / 2.0, 0.0),
                    scale: vec3(2.0, 2.0, 0.0),
                    rotation: Quat::from_rotation_z((180.0 as f32).to_radians()),
                },
                ..default()
            })
            .insert(Blocker)
            .id();
        let child_bottom = commands
            .spawn_bundle(SpriteBundle {
                texture: pipe_bottom,
                ..default()
            })
            .insert_bundle(TransformBundle {
                local: Transform {
                    translation: vec3(0.0, -(PIPE_HEIGHT + space_between / 2.0), 0.0),
                    scale: vec3(2.0, 2.0, 0.0),
                    ..default()
                },
                ..default()
            })
            .insert(Blocker)
            .id();
        commands
            .entity(parent)
            .push_children(&[child_top, child_bottom]);
    }
}

fn setup_floor(mut commands: Commands, asset_server: Res<AssetServer>) {
    let quotient = FLOOR_WIDTH / SCREEN_WIDTH;
    let floors = f32::ceil(1.5 / quotient) as i32;

    let base_image = asset_server.load("sprites/base.png");
    for n in 0..floors {
        let floor_img = base_image.clone();
        commands
            .spawn_bundle(SpriteBundle {
                texture: floor_img,
                transform: Transform {
                    scale: vec3(1.0, 1.0, 0.0),
                    translation: vec3(
                        -(SCREEN_WIDTH / 2.0) + FLOOR_WIDTH / 2.0 + (n as f32) * FLOOR_WIDTH,
                        FLOOR_POS,
                        1.0,
                    ),
                    ..default()
                },
                ..default()
            })
            .insert(AutoMoving {
                width: FLOOR_WIDTH,
                displacement: 0.0,
                initial: vec3(0.0, FLOOR_POS, 1.0),
                randomness: vec3(0.0, 0.0, 0.0),
            })
            .insert(Blocker);
    }
}

fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    let bg_image = asset_server.load("sprites/background-day.png");
    commands.spawn_bundle(SpriteBundle {
        texture: bg_image,
        transform: Transform {
            scale: vec3(0.5 * SCALE, 0.5 * SCALE, 0.0),
            ..default()
        },
        ..default()
    });

    let texture_handle = asset_server.load("sprites/redbird.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, vec2(34.0, 24.0), 3, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let bird_xy = vec3(PLAYER_POS_X, 0.0, 1.0);
    commands
        .spawn()
        .insert(Player {
            movement_speed: 0.0,
            angle: 0.0,
        })
        .insert_bundle(SpriteSheetBundle {
            transform: Transform {
                translation: bird_xy,
                scale: BIRD_SIZE,
                ..default()
            },
            texture_atlas: texture_atlas_handle,
            ..default()
        })
        .insert(Collider)
        .insert(AnimationTimer(Timer::from_seconds(0.2, true)));
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Flappy bird in Rust".to_string(),
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
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
        .add_system_set(SystemSet::on_update(GameState::Paused).with_system(handle_menu_input))
        .add_system_set(
            SystemSet::on_update(GameState::Running)
                .with_system(player_movement_system)
                .with_system(animate_sprite_system)
                .with_system(auto_move_system)
                .with_system(collision_system)
                .with_system(point_count_system)
                .with_system(update_score_text)
                .with_system(handle_input_system),
        )
        .run();
}
