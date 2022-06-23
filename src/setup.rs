use crate::components::*;
use bevy::{
    math::{vec2, vec3},
    prelude::*,
};
use rand::prelude::*;

pub fn setup_font(mut commands: Commands, asset_server: Res<AssetServer>) {
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
            transform: Transform::from_xyz(0.0, SCREEN.y / 4.0, 1.0),
            ..default()
        })
        .insert(ScoreText);
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
        let space_between = SCREEN.y / 3.5;
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

pub fn setup_floor(mut commands: Commands, asset_server: Res<AssetServer>) {
    let quotient = FLOOR_WIDTH / SCREEN.x;
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
                        -(SCREEN.x / 2.0) + FLOOR_WIDTH / 2.0 + (n as f32) * FLOOR_WIDTH,
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
