use bevy::{
    core::FixedTimestep,
    math::{const_vec3, vec2, vec3},
    prelude::*,
    window::PresentMode,
};

const SCREEN_HEIGHT: f32 = 960.0;
const SCREEN_WIDTH: f32 = 640.0;

const TIME_STEP: f32 = 1.0 / 60.0;

const SCOREBOARD_FONT_SIZE: f32 = 40.0;
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);

// unscaled consts
const GAME_WIDTH: f32 = 136.0;
const SCALE: f32 = SCREEN_WIDTH / GAME_WIDTH;
const PLAYER_HEIGHT: f32 = 12.0;
const PLAYER_WIDTH: f32 = 16.0;

const PIXELS_PER_METER: f32 = 30.0 / SCALE;
const JUMP_SPEED: f32 = 75.0 * PIXELS_PER_METER * SCALE;
const SCALED_GRAVITY: f32 = -9.81 * PIXELS_PER_METER * SCALE;

const BIRD_SIZE: Vec3 = const_vec3!([0.5 * SCALE, 0.5 * SCALE, 0.0]);

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

    if mouse_button_input.just_pressed(MouseButton::Left)
        || keyboard_input.just_pressed(KeyCode::Space)
    {
        player.movement_speed = JUMP_SPEED;
    }

    for _touch in touches.iter_just_pressed() {
        player.movement_speed = JUMP_SPEED;
    }
}

fn player_movement_system(
    time: Res<Time>,
    gravity: Res<Gravity>,
    mut query: Query<(&mut Player, &mut Transform)>,
) {
    let (mut player, mut transform) = query.single_mut();

    let new_speed = player.movement_speed + gravity.0;
    player.movement_speed = new_speed.clamp(-1000.0, 10000.0);

    let new_player_pos =
        transform.translation.y + vec3(0.0, player.movement_speed * time.delta_seconds(), 0.0);
    let bound = SCREEN_HEIGHT / 2.0 - PLAYER_HEIGHT * SCALE;
    transform.translation =
        new_player_pos.clamp(vec3(0.0, -1.0 * bound, 0.0), vec3(0.0, bound, 0.0));

    if player.movement_speed > 0.0 {
        player.angle = 45.0;
    } else {
        player.angle = (player.angle - 180.0 * time.delta_seconds()).clamp(-90.0, 45.0);
    }

    transform.rotation = Quat::from_rotation_z(f32::to_radians(player.angle));
}

// fn animate_player_system(time: Res<Time>, texture_atlases: Res<Assets<Text) {}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    let texture_handle = asset_server.load("sprites/redbird.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, vec2(34.0, 24.0), 3, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let bird_xy = vec3(0.0, 0.0, 1.0);
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
        .insert(AnimationTimer(Timer::from_seconds(0.1, false)));
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
        .run();
    println!("Hello, world!");
}
