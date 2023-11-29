use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

use crate::consts;

#[derive(Component)]
struct Zombie;

#[derive(Component)]
struct Spawner;

use super::WinFlag;

use super::{despawn_screen, DisplayQuality, GameState, OnGameScreen, Volume, TEXT_COLOR};

pub struct PlatformsPlugin;

impl Plugin for PlatformsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), setup)
            .add_systems(Update, spawn_zombies.run_if(in_state(GameState::Game)))
            .add_systems(Update, despawn_zombies.run_if(in_state(GameState::Game)))
            // .add_systems(Update, spawn_flag.run_if(in_state(GameState::Game)))
            // .add_systems(Update, despawn_flag.run_if(in_state(GameState::Game)))
            .add_systems(OnExit(GameState::Game), despawn_screen::<Zombie>)
            .add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>);
    }
}

#[derive(Bundle)]
struct PlatformBundle {
    sprite_bundle: SpriteBundle,
    body: RigidBody,
    collider: Collider,
}

impl PlatformBundle {
    fn new(translation: Vec3, scale: Vec3, collider: Collider) -> (Self, OnGameScreen) {
        (
            Self {
                sprite_bundle: SpriteBundle {
                    sprite: Sprite {
                        color: consts::COLOR_FLOOR,
                        ..Default::default()
                    },
                    transform: Transform {
                        translation,
                        scale,
                        ..Default::default()
                    },
                    ..Default::default()
                },
                body: RigidBody::Fixed,
                collider,
            },
            OnGameScreen,
        )
    }
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // commands.spawn(Camera2dBundle::default());

    // Cube
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Cube::new(50.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
            transform: Transform::from_translation(Vec3::new(50., 0., 0.)),
            ..default()
        },
        Spawner,
        OnGameScreen,
        RigidBody::Fixed,
        Collider::cuboid(25., 25.),
    ));

    // Bottom floor
    commands.spawn(PlatformBundle::new(
        Vec3::new(
            0.0,
            consts::WINDOW_BOTTOM_Y + (consts::FLOOR_THICKNESS / 2.0),
            0.0,
        ),
        Vec3::new(consts::WINDOW_WIDTH, consts::FLOOR_THICKNESS, 1.0),
        Collider::cuboid(0.5, 0.5),
    ));

    // Left floor
    commands.spawn(PlatformBundle::new(
        Vec3::new(
            -(consts::WINDOW_WIDTH / 2.0) + (consts::FLOOR_THICKNESS / 2.0),
            0.,
            0.0,
        ),
        Vec3::new(consts::FLOOR_THICKNESS, consts::WINDOW_HEIGHT, 1.0),
        Collider::cuboid(0.5, 0.5),
    ));

    // Right floor
    commands.spawn(PlatformBundle::new(
        Vec3::new(
            (consts::WINDOW_WIDTH / 2.0) - (consts::FLOOR_THICKNESS / 2.0),
            0.,
            0.0,
        ),
        Vec3::new(consts::FLOOR_THICKNESS, consts::WINDOW_HEIGHT, 1.0),
        Collider::cuboid(0.5, 0.5),
    ));

    // Top floor
    commands.spawn(PlatformBundle::new(
        Vec3::new(
            0.0,
            -consts::WINDOW_BOTTOM_Y - (consts::FLOOR_THICKNESS / 2.0),
            0.0,
        ),
        Vec3::new(consts::WINDOW_WIDTH, consts::FLOOR_THICKNESS, 1.0),
        Collider::cuboid(0.5, 0.5),
    ));

    let flag: bevy::prelude::Handle<Image> = asset_server.load("texture/flag.png");
    commands
        .spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: consts::COLOR_FLOOR,
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(
                        consts::WINDOW_LEFT_X + 1480.0,
                        consts::WINDOW_BOTTOM_Y + 800.0,
                        0.0,
                    ),
                    scale: Vec3::new(0.2, 0.2, 1.0),
                    ..Default::default()
                },
                texture: flag,
                ..Default::default()
            },
            OnGameScreen,
            WinFlag,
            Collider::cuboid(124., 124.),
        ))
        .insert(Sensor);
    // .insert(RigidBody::Fixed)
}

fn spawn_zombies(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Spawner>>,
) {
    if time.elapsed_seconds().round() as i32 % 2 == 0 && time.elapsed_seconds() > 1. {
        for mut transform in &mut query {
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Cube::new(25.).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
                    transform: Transform::from_translation(Vec3::new(
                        transform.translation.x + 37.5,
                        transform.translation.y,
                        0.,
                    )),
                    ..default()
                },
                Zombie,
                RigidBody::Fixed,
                Collider::cuboid(12.5, 12.5),
            ));
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Cube::new(25.).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
                    transform: Transform::from_translation(Vec3::new(
                        transform.translation.x - 37.5,
                        transform.translation.y,
                        0.,
                    )),
                    ..default()
                },
                Zombie,
                RigidBody::Fixed,
                Collider::cuboid(12.5, 12.5),
            ));
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Cube::new(25.).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
                    transform: Transform::from_translation(Vec3::new(
                        transform.translation.x,
                        transform.translation.y + 37.5,
                        0.,
                    )),
                    ..default()
                },
                Zombie,
                RigidBody::Fixed,
                Collider::cuboid(12.5, 12.5),
            ));
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Cube::new(25.).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
                    transform: Transform::from_translation(Vec3::new(
                        transform.translation.x,
                        transform.translation.y - 37.5,
                        0.,
                    )),
                    ..default()
                },
                Zombie,
                RigidBody::Fixed,
                Collider::cuboid(12.5, 12.5),
            ));
        }
    }
}

fn despawn_zombies(
    mut commands: Commands,
    time: Res<Time>,
    mut entities: Query<Entity, With<Zombie>>,
) {
    if time.elapsed_seconds().round() as i32 % 3 == 0 && time.elapsed_seconds() > 1. {
        for entity in &mut entities {
            commands.entity(entity).despawn();
        }
    }
}
