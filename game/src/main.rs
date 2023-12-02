#![allow(clippy::type_complexity)]

use bevy::{prelude::*, window::WindowResolution};
use bevy_rapier2d::prelude::*;

mod audio;
mod consts;
mod environment;
mod menu;
mod player;
mod splash;

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

impl Volume {
    fn get_val(self) -> f64 {
        self.0 as f64 / 10.0
    }
}

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
        .add_plugins(RapierDebugRenderPlugin::default())

        .insert_resource(DisplayQuality::Medium)
        .insert_resource(Volume(7))
        .add_state::<GameState>()

        .add_systems(Startup, setup)
        
        .add_plugins(splash::SplashPlugin)
        .add_plugins(menu::MenuPlugin)
        .add_plugins(environment::PlatformsPlugin)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(audio::GameAudioPlugin)

        .add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
