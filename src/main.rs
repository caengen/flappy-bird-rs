use bevy::{
    core::FixedTimestep,
    math::{const_vec2, const_vec3},
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

const SCREEN_HEIGHT: f32 = 960.0;
const SCREEN_WIDTH: f32 = 640.0;

// unscaled consts
const GAME_WIDTH: f32 = 135.0;
const PLAYER_WIDTH = 17.0;

const WORLD_BASE_SPEED = 10.0;

fn main() {
    println!("Hello, world!");
}
`