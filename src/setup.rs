use crate::components::*;
use bevy::{
    ecs::query::QueryIter,
    math::{vec2, vec3},
    prelude::*,
    utils::Duration,
};
use rand::prelude::*;

pub fn setup_font(mut commands: Commands, asset_server: Res<AssetServer>) {
    let shadow_font = asset_server.load("flappy-font.ttf");
    let font = shadow_font.clone();
    let shadow_style = TextStyle {
        font: shadow_font,
        font_size: 100.0,
        color: Color::BLACK,
    };
    let style = TextStyle {
        font,
        font_size: 100.0,
        color: Color::WHITE,
    };
    let alignment = TextAlignment {
        vertical: VerticalAlign::Center,
        horizontal: HorizontalAlign::Center,
    };

    //shadow
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section("0", shadow_style, alignment),
            transform: Transform::from_xyz(5.0, SCREEN.y / 4.0 - 5.0, 10.0),
            ..default()
        })
        .insert(ScoreText);
    commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section("0", style, alignment),
            transform: Transform::from_xyz(0.0, SCREEN.y / 4.0, 11.0),
            ..default()
        })
        .insert(ScoreText);
}

pub fn setup_game_over_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let image = asset_server.load("sprites/gameover.png");

    commands.spawn().insert(GameOverUIInputTimer(Timer::new(
        Duration::from_millis(500),
        false,
    )));

    commands
        .spawn_bundle(SpriteBundle {
            texture: image,
            transform: Transform {
                translation: vec3(0.0, 0.0, 10.0),
                scale: vec3(2.0, 2.0, 1.0),
                ..default()
            },
            visibility: Visibility { is_visible: false },
            ..default()
        })
        .insert(GameOverUI);
}

pub fn set_game_over_ui_visible(
    mut query: Query<(&GameOverUI, &mut Visibility)>,
    mut timer_query: Query<(Entity, &mut GameOverUIInputTimer)>,
) {
    for (_, mut timer) in timer_query.iter_mut() {
        timer.0.reset();
    }

    for (_, mut visibility) in query.iter_mut() {
        visibility.is_visible = true;
    }
}
pub fn set_game_over_ui_hidden(mut query: Query<(&GameOverUI, &mut Visibility)>) {
    for (_, mut visibility) in query.iter_mut() {
        visibility.is_visible = false;
    }
}

pub fn setup_pipes(mut commands: Commands, asset_server: Res<AssetServer>) {
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
                        n as f32 * (PIPE_OPENING_Y_POS_FACTOR * rand_num),
                        1.0,
                    ),
                    ..Default::default()
                },
                ..default()
            })
            .insert(AutoMoving {
                width: PIPE.x * 2.0,
                displacement: SPACE_BETWEEN_PIPES / 2.0,
                randomness: vec3(0.0, PIPE_OPENING_Y_POS_FACTOR, 0.0),
                initial: vec3(0.0, 0.0, 0.0),
            })
            .insert(Countable(true))
            .id();
        let pipe_top = pipe_handle.clone();
        let pipe_bottom = pipe_top.clone();
        let child_top = commands
            .spawn_bundle(SpriteBundle {
                texture: pipe_top,
                ..default()
            })
            .insert_bundle(TransformBundle {
                local: Transform {
                    translation: vec3(0.0, PIPE.y / 2.0 + VERTICAL_SPACE_BETWEEN_PIPES / 2.0, 0.0),
                    scale: vec3(2.0, 2.0, 1.0),
                    rotation: Quat::from_rotation_z((180.0 as f32).to_radians()),
                },
                ..default()
            })
            .insert(Blocker(PIPE))
            .id();

        let child_bottom = commands
            .spawn_bundle(SpriteBundle {
                texture: pipe_bottom,
                ..default()
            })
            .insert_bundle(TransformBundle {
                local: Transform {
                    translation: vec3(
                        0.0,
                        -(PIPE.y / 2.0) - (VERTICAL_SPACE_BETWEEN_PIPES / 2.0),
                        0.0,
                    ),
                    scale: vec3(2.0, 2.0, 1.0),
                    ..default()
                },
                ..default()
            })
            .insert(Blocker(PIPE))
            .id();

        commands
            .entity(parent)
            .push_children(&[child_top, child_bottom]);
    }
}

pub fn setup_floor(mut commands: Commands, asset_server: Res<AssetServer>) {
    let quotient = SCREEN.x / FLOOR_WIDTH;
    let floors = f32::ceil(1.5 * quotient) as i32;

    let base_image = asset_server.load("sprites/base.png");
    for n in 0..floors {
        let floor_img = base_image.clone();
        commands
            .spawn_bundle(SpriteBundle {
                texture: floor_img,
                transform: Transform {
                    translation: vec3(
                        -(SCREEN.x / 2.0) + FLOOR.x / 2.0 + (n as f32) * FLOOR.x,
                        FLOOR_POS,
                        3.0,
                    ),
                    scale: vec3(1.0, 1.0, 1.0),
                    ..default()
                },
                ..default()
            })
            .insert(SpeedAnimated { width: FLOOR.x })
            // compensating for collision not taking rotation into account?
            .insert(Blocker(FLOOR * 1.2));
    }
}

pub fn setup_player(
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
            scale: PLAYER_SCALE,
            ..default()
        },
        ..default()
    });

    let texture_handle = asset_server.load("sprites/redbird.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, vec2(34.0, 24.0), 3, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let bird_xy = vec3(PLAYER_POS_X, 0.0, 2.0);
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
        .insert(AnimationTimer(Timer::from_seconds(0.15, true)));
}

pub fn game_over_cleanup(
    _: EventReader<ResetGameEvent>,
    mut scoreboard: ResMut<Scoreboard>,
    mut player_query: Query<(&Player, &mut Transform)>,
    mut pipe_query: Query<(&mut Countable, &mut Transform, Without<Player>)>,
    mut text_query: Query<(&ScoreText, &mut Text)>,
) {
    scoreboard.score = 0;
    for (_, mut text) in text_query.iter_mut() {
        text.sections.get_mut(0).unwrap().value = "0".to_string();
    }

    let mut rng = thread_rng();
    let mut n = 0;
    for (mut countable, mut pipe_transform, _) in pipe_query.iter_mut() {
        let rand_num = if rng.gen_ratio(1, 2) {
            rng.gen_range(0.5..1.0)
        } else {
            rng.gen_range(-1.0..-0.5)
        };
        pipe_transform.translation = vec3(
            PIPE_START_X + n as f32 * SPACE_BETWEEN_PIPES,
            n as f32 * (PIPE_OPENING_Y_POS_FACTOR * rand_num),
            1.0,
        );
        countable.0 = true;

        n += 1;
    }

    let (_, mut player_transform) = player_query.single_mut();
    player_transform.translation = vec3(PLAYER_POS_X, 0.0, 2.0);
    player_transform.rotation = Quat::from_rotation_z(f32::to_radians(0.0));
}
