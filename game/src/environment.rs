use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

use crate::consts;

use rand::Rng;

#[derive(Component)]
struct Zombie;

#[derive(Component)]
struct Spawner;

use super::{despawn_screen, GameState, OnGameScreen, WinFlag};

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
enum SpawnStatus {
    Ready,
    Standby,
}

pub struct PlatformsPlugin;

impl Plugin for PlatformsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), setup)
            .insert_resource(SpawnStatus::Ready)
            .add_systems(Update, spawn_zombies.run_if(in_state(GameState::Game)))
            .add_systems(Update, despawn_zombies.run_if(in_state(GameState::Game)))
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

fn overlaps(fst: (f32, f32), snd: (f32, f32)) -> bool {
    return (fst.0 - 30. < snd.0 && snd.0 < fst.0 + 30.)
        || (fst.1 - 30. < snd.1 && snd.1 < fst.1 + 30.);
}

fn is_invalid_spot(spots: Vec<(f32, f32)>, new_spot: (f32, f32)) -> bool {
    for spot in spots {
        if overlaps(spot, new_spot) {
            return true;
        }
    }
    false
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let mut spots: Vec<(f32, f32)> = vec![
        //flag
        (
            consts::WINDOW_LEFT_X + 1480.0,
            consts::WINDOW_BOTTOM_Y + 800.0,
        ),
        //player
        (
            consts::WINDOW_LEFT_X + 730.0,
            consts::WINDOW_BOTTOM_Y + 50.0,
        ),
    ];
    let mut rng = rand::thread_rng();
    for c in 1..=60 {
        let mut x: f32 = rng.gen_range(consts::WINDOW_LEFT_X+25. ..=consts::WINDOW_WIDTH/2. -25.);
        let mut y: f32 = rng.gen_range(consts::WINDOW_BOTTOM_Y +25. ..=consts::WINDOW_HEIGHT/2. -25.);
        let is_spawner: i32 = rng.gen_range(0..=1);

        let mut i = 0;
        while is_invalid_spot(spots.clone(), (x, y)) {
            x = rng.gen_range(consts::WINDOW_LEFT_X+25. ..=consts::WINDOW_WIDTH/2. -25.);
            y = rng.gen_range(consts::WINDOW_BOTTOM_Y+25. ..=consts::WINDOW_HEIGHT/2. -25.);
            i += 1;
            if i > 100 {
                break;
            }
        }

        if i > 100 {
            continue;
        }

        if is_spawner == 1 {
            spots.push((x, y));
        }
        println!("Accept {}", c);

        // Cube
        let cube_template = MaterialMesh2dBundle {
            mesh: meshes.add(shape::Cube::new(50.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
            transform: Transform::from_translation(Vec3::new(x, y, 0.)),
            ..default()
        };

        if is_spawner == 0 {
            commands.spawn((
                cube_template,
                OnGameScreen,
                RigidBody::Fixed,
                Collider::cuboid(25., 25.),
            ));
        } else {
            commands.spawn((
                cube_template,
                Spawner,
                OnGameScreen,
                RigidBody::Fixed,
                Collider::cuboid(25., 25.),
            ));
        }
    }

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
        .insert(Sensor)
        .insert(RigidBody::Dynamic);
}

fn spawn_zombies(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Spawner>>,
    mut sstatus: ResMut<SpawnStatus>,
) {
    if time.elapsed_seconds().round() as i32 % 2 == 0
        && time.elapsed_seconds() > 1.
        && *sstatus == SpawnStatus::Ready
    {
        *sstatus = SpawnStatus::Standby;
        for transform in &mut query {
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Cube::new(25.).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::RED)),
                    transform: Transform::from_translation(Vec3::new(
                        transform.translation.x + 37.5,
                        transform.translation.y,
                        0.,
                    )),
                    ..default()
                },
                Zombie,
                RigidBody::KinematicVelocityBased,
                Collider::cuboid(12.5, 12.5),
                Velocity {
                    linvel: Vec2::new(20.0, 0.0),
                    angvel: 0.,
                },
            ));
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Cube::new(25.).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::RED)),
                    transform: Transform::from_translation(Vec3::new(
                        transform.translation.x - 37.5,
                        transform.translation.y,
                        0.,
                    )),
                    ..default()
                },
                Zombie,
                RigidBody::KinematicVelocityBased,
                Collider::cuboid(12.5, 12.5),
                Velocity {
                    linvel: Vec2::new(-20.0, 0.0),
                    angvel: 0.,
                },
            ));
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Cube::new(25.).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::RED)),
                    transform: Transform::from_translation(Vec3::new(
                        transform.translation.x,
                        transform.translation.y + 37.5,
                        0.,
                    )),
                    ..default()
                },
                Zombie,
                RigidBody::KinematicVelocityBased,
                Collider::cuboid(12.5, 12.5),
                Velocity {
                    linvel: Vec2::new(0.0, 20.0),
                    angvel: 0.,
                },
            ));
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes.add(shape::Cube::new(25.).into()).into(),
                    material: materials.add(ColorMaterial::from(Color::RED)),
                    transform: Transform::from_translation(Vec3::new(
                        transform.translation.x,
                        transform.translation.y - 37.5,
                        0.,
                    )),
                    ..default()
                },
                Zombie,
                RigidBody::KinematicVelocityBased,
                Collider::cuboid(12.5, 12.5),
                Velocity {
                    linvel: Vec2::new(0.0, -20.0),
                    angvel: 0.,
                },
            ));
        }
    }
}

fn despawn_zombies(
    mut commands: Commands,
    time: Res<Time>,
    mut entities: Query<Entity, With<Zombie>>,
    mut sstatus: ResMut<SpawnStatus>,
) {
    if time.elapsed_seconds().round() as i32 % 3 == 0 && time.elapsed_seconds() > 1. {
        *sstatus = SpawnStatus::Ready;
        for entity in &mut entities {
            commands.entity(entity).despawn();
        }
    }
}
