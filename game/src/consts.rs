use bevy::prelude::*;
// use bevy_rapier2d::prelude::*;

pub const WINDOW_WIDTH: f32 = 1536.0;
pub const WINDOW_HEIGHT: f32 = 864.0;

pub const WINDOW_BOTTOM_Y: f32 = WINDOW_HEIGHT / -2.0;
pub const WINDOW_LEFT_X: f32 = WINDOW_WIDTH / -2.0;

pub const FLOOR_THICKNESS: f32 = 1.0;
pub const COLOR_FLOOR: Color = Color::rgb(0.45, 0.55, 0.66);

pub const PLAYER_VELOCITY_X: f32 = 150.0;

pub const SPRTPL_COLS: usize = 9;
pub const SPRTPL_ROWS: usize = 4;

pub const SPRTPL_W: f32 = 33.0;
pub const SPRTPL_H: f32 = 52.0;
