#![allow(clippy::type_complexity)]

use bevy::{prelude::*, window::WindowResolution};
use bevy_rapier2d::prelude::*;

mod consts;
mod environment;
mod menu;
mod player;
mod splash;
// mod game_setup;

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

// Enum that will be used as a global state for the game
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Splash,
    Menu,
    Game,
}

// One of the two settings that can be set through the menu. It will be a resource in the app
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
enum DisplayQuality {
    Low,
    Medium,
    High,
}

// One of the two settings that can be set through the menu. It will be a resource in the app
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
struct Volume(u32);

#[derive(Component)]
struct OnGameScreen;

#[derive(Component)]
struct WinFlag;

#[derive(Component)]
struct PlayerFlag;

#[derive(Resource, Deref, DerefMut)]
struct GameTimer(Timer);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Passive Power".to_string(),
                resolution: WindowResolution::new(consts::WINDOW_WIDTH, consts::WINDOW_HEIGHT),
                resizable: false,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1e-12)) // Physics plugin
        .add_plugins(RapierDebugRenderPlugin::default()) // Debug plugin
        .insert_resource(DisplayQuality::Medium)
        .insert_resource(Volume(7))
        // Declare the game state, whose starting value is determined by the `Default` trait
        .add_state::<GameState>()
        .add_systems(Startup, setup)
        .add_plugins(splash::SplashPlugin)
        .add_plugins(menu::MenuPlugin)
        // .add_systems(OnEnter(GameState::Game), game_setup)
        // .add_systems(Update, game.run_if(in_state(GameState::Game)))
        .add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>)
        .add_plugins(environment::PlatformsPlugin)
        .add_plugins(player::PlayerPlugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

// fn game_setup(mut commands: Commands, _display_quality: Res<DisplayQuality>, _volume: Res<Volume>) {
// commands.insert_resource(GameTimer(Timer::from_seconds(1000.0, TimerMode::Once)));
// }

// fn game(
//     time: Res<Time>,
//     mut game_state: ResMut<NextState<GameState>>,
//     mut timer: ResMut<GameTimer>,
// ) {
//     if timer.tick(time.delta()).finished() {
//         game_state.set(GameState::Menu);
//     }
// }

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
