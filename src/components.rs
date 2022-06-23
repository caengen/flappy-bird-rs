use bevy::{
    math::{const_vec2, const_vec3},
    prelude::*,
};

pub const SCREEN_HEIGHT: f32 = 960.0;
pub const SCREEN_WIDTH: f32 = 640.0;

pub const FLOOR_POS: f32 = -112.0 * 4.0;
pub const AUTO_MOVE_SPEED: f32 = 1.0 * PIXELS_PER_METER;

pub const TIME_STEP: f32 = 1.0 / 60.0;

pub const SCOREBOARD_FONT_SIZE: f32 = 40.0;
pub const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);

// unscaled consts
pub const GAME_WIDTH: f32 = 136.0;
pub const SCALE: f32 = SCREEN_WIDTH / GAME_WIDTH;
pub const PLAYER_HEIGHT: f32 = 12.0;
pub const PLAYER_WIDTH: f32 = 16.0;
pub const PLAYER_DIM: Vec2 = const_vec2!([PLAYER_WIDTH, PLAYER_HEIGHT]);
pub const PLAYER_POS_X: f32 = -75.0;

pub const PIXELS_PER_METER: f32 = 30.0 / SCALE;
pub const JUMP_SPEED: f32 = 125.0 * PIXELS_PER_METER;
pub const SCALED_GRAVITY: f32 = -9.81 * PIXELS_PER_METER;

pub const BIRD_SIZE: Vec3 = const_vec3!([0.5 * SCALE, 0.5 * SCALE, 1.0]);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    Paused,
    Running,
    GameOver,
}
pub struct Scoreboard {
    pub score: usize,
}

// in m/s - usually -9.8 m/s
pub struct Gravity(pub f32);

// unscaled
pub const PIPE_WIDTH: f32 = 52.0;
pub const PIPE_HEIGHT: f32 = 320.0;
//scaled
pub const PIPE_DIM: Vec2 = const_vec2!([PIPE_WIDTH * 2.0, PIPE_HEIGHT * 2.0]);
pub const SPACE_BETWEEN_PIPES: f32 = 75.0 * PIXELS_PER_METER;
pub const PIPE_START_X: f32 = SCREEN_WIDTH + PIPE_WIDTH;
pub const PIPE_RANDOM_Y: f32 = 40.0 * PIXELS_PER_METER;

#[derive(Component)]
pub struct Pipe;

pub const FLOOR_WIDTH: f32 = 336.0;
pub const FLOOR_HEIGHT: f32 = 112.0;

#[derive(Component)]
pub struct Countable(pub bool);

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct Collider;
#[derive(Component)]
pub struct Blocker;

pub struct CollisionEvent;

#[derive(Component)]
pub struct Floor;
#[derive(Component)]
pub struct Player {
    // in m/s
    pub movement_speed: f32,
    pub angle: f32,
}

#[derive(Component)]
pub struct AutoMoving {
    pub width: f32,
    pub displacement: f32,
    pub randomness: Vec3,
    pub initial: Vec3,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);
