use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

use game::consts;

pub struct PlatformsPlugin;

impl Plugin for PlatformsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

#[derive(Bundle)]
struct PlatformBundle {
    sprite_bundle: SpriteBundle,
    body: RigidBody,
    collider: Collider,
}

impl PlatformBundle {
    fn new(translation: Vec3, scale: Vec3, collider: Collider) -> Self {
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
        }
    }
}

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // commands.spawn(Camera2dBundle::default());

    // Circle
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(50.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: Transform::from_translation(Vec3::new(-150., 100., 0.)),
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(25., 25.));

    //Moving object
    // commands
    //     .spawn(MaterialMesh2dBundle {
    //         mesh: meshes.add(shape::Circle::default().into()).into(),
    //         material: materials.add(ColorMaterial::from(Color::PURPLE)),
    //         transform: Transform {
    //             translation: Vec3::new(consts::WINDOW_LEFT_X + 100.0, consts::WINDOW_BOTTOM_Y + 30.0, 0.0),
    //             scale: Vec3::new(30.0, 30.0, 1.0),
    //             ..Default::default()
    //         },
    //         ..default()
    //     })
    //     .insert(RigidBody::KinematicPositionBased)
    //     .insert(Collider::ball(0.5))
    //     .insert(KinematicCharacterController::default());

    // Rectangle
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(50.0, 100.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(-50., -100., 100.)),
        ..default()
    }).insert(RigidBody::Fixed)
    .insert(Collider::cuboid(25., 25.));

    // Quad
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(shape::Quad::new(Vec2::new(50., 100.)).into())
            .into(),
        material: materials.add(ColorMaterial::from(Color::LIME_GREEN)),
        transform: Transform::from_translation(Vec3::new(50., 0., 0.)),
        ..default()
    });

    // Hexagon
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::RegularPolygon::new(50., 6).into()).into(),
        material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
        transform: Transform::from_translation(Vec3::new(150., 0., 0.)),
        ..default()
    });

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
}
