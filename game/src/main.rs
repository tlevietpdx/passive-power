use bevy::{prelude::*, window::WindowResolution};
use bevy_rapier2d::prelude::*;

mod consts;
mod environment;
mod player;

fn main() {
    // App::new()
    //     .add_plugins(DefaultPlugins)
    //     .add_systems(Startup, setup)
    //     .run();

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
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(10.)) // Physics plugin
        .add_plugins(RapierDebugRenderPlugin::default()) // Debug plugin
        .add_systems(Startup, setup)
        .add_plugins(environment::PlatformsPlugin)
        .add_plugins(player::PlayerPlugin)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
