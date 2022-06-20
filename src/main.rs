use bevy::{
    core::FixedTimestep,
    math::{const_vec3, vec2, vec3},
    prelude::*,
    window::PresentMode,
};

const SCREEN_HEIGHT: f32 = 960.0;
const SCREEN_WIDTH: f32 = 640.0;

const FLOOR_POS: f32 = -112.0 * 4.0;
const FLOOR_SPEED: f32 = 1.0 * PIXELS_PER_METER;

const TIME_STEP: f32 = 1.0 / 60.0;

const SCOREBOARD_FONT_SIZE: f32 = 40.0;
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);

// unscaled consts
const GAME_WIDTH: f32 = 136.0;
const SCALE: f32 = SCREEN_WIDTH / GAME_WIDTH;
const PLAYER_HEIGHT: f32 = 12.0;
const PLAYER_WIDTH: f32 = 16.0;
const PLAYER_POS_X: f32 = -75.0;

const PIXELS_PER_METER: f32 = 30.0 / SCALE;
const JUMP_SPEED: f32 = 100.0 * PIXELS_PER_METER;
const SCALED_GRAVITY: f32 = -9.81 * PIXELS_PER_METER;

const BIRD_SIZE: Vec3 = const_vec3!([0.5 * SCALE, 0.5 * SCALE, 1.0]);

struct Scoreboard {
    score: usize,
}

// in m/s - usually -9.8 m/s
struct Gravity(f32);

#[derive(Component)]
struct Player {
    // in m/s
    movement_speed: f32,
    angle: f32,
}

#[derive(Component)]
struct Floor;

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn handle_input_system(
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

fn animate_floor_system(mut query: Query<(&Floor, &mut Transform)>) {
    for (_, mut transform) in query.iter_mut() {
        transform.translation.x -= FLOOR_SPEED;

        if transform.translation.x <= -504.0 {
            transform.translation.x = 504.0
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    let bg_image = asset_server.load("sprites/background-day.png");
    let base_image = asset_server.load("sprites/base.png");
    let base_image_2 = base_image.clone();
    let base_image_3 = base_image.clone();
    commands.spawn_bundle(SpriteBundle {
        texture: bg_image,
        transform: Transform {
            scale: vec3(0.5 * SCALE, 0.5 * SCALE, 0.0),
            ..default()
        },
        ..default()
    });
    commands
        .spawn_bundle(SpriteBundle {
            texture: base_image,
            transform: Transform {
                scale: vec3(1.0, 1.0, 0.0),
                translation: vec3(-168.0, FLOOR_POS, 0.0),
                ..default()
            },
            ..default()
        })
        .insert(Floor);
    commands
        .spawn_bundle(SpriteBundle {
            texture: base_image_2,
            transform: Transform {
                scale: vec3(1.0, 1.0, 0.0),
                translation: vec3(168.0, FLOOR_POS, 0.0),
                ..default()
            },
            ..default()
        })
        .insert(Floor);
    commands
        .spawn_bundle(SpriteBundle {
            texture: base_image_3,
            transform: Transform {
                scale: vec3(1.0, 1.0, 0.0),
                translation: vec3(337.0, FLOOR_POS, 0.0),
                ..default()
            },
            ..default()
        })
        .insert(Floor);

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
        .add_startup_system(setup)
        .add_system(handle_input_system)
        .add_system(player_movement_system)
        .add_system(animate_sprite_system)
        .add_system(animate_floor_system)
        .run();
}
