use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

use std::time::Duration;

use game::animator::{Animation, AnimationPlugin};
use game::consts;

const P_WALK_U: &[usize] = &[0, 1, 2, 3, 4, 5, 6, 7, 8];
const P_WALK_L: &[usize] = &[9, 10, 11, 12, 13, 14, 15, 16, 17];
const P_WALK_D: &[usize] = &[18, 19, 20, 21, 22, 23, 24, 25, 26];
const P_WALK_R: &[usize] = &[27, 28, 29, 30, 31, 32, 33, 34, 35];
const P_IDLE: usize = 18;

const CYCLE_DELAY: Duration = Duration::from_millis(100);

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player)
            .add_systems(Update, movement)
            .add_systems(Update, anim_idle)
            .add_systems(Update, anim_mov_d)
            .add_systems(Update, anim_mov_u)
            .add_systems(Update, anim_mov_r)
            .add_systems(Update, anim_mov_l)
            .add_plugins(AnimationPlugin);
    }
}

fn anim_mov_r(
    mut commands: Commands,
    query: Query<(Entity, &KinematicCharacterControllerOutput), Without<Animation>>,
) {
    if query.is_empty() {
        return;
    }

    let (player, output) = query.single();
    if output.effective_translation.x > 0.0 {
        commands.entity(player).remove::<Animation>();
        commands
            .entity(player)
            .insert(Animation::new(P_WALK_R, CYCLE_DELAY));
    }
}

fn anim_mov_l(
    mut commands: Commands,
    query: Query<(Entity, &KinematicCharacterControllerOutput), Without<Animation>>,
) {
    if query.is_empty() {
        return;
    }

    let (player, output) = query.single();
    if output.effective_translation.x < 0.0 {
        commands.entity(player).remove::<Animation>();
        commands
            .entity(player)
            .insert(Animation::new(P_WALK_L, CYCLE_DELAY));
    }
}

fn anim_mov_u(
    mut commands: Commands,
    query: Query<(Entity, &KinematicCharacterControllerOutput), Without<Animation>>,
) {
    if query.is_empty() {
        return;
    }

    let (player, output) = query.single();
    if output.effective_translation.y > 0.0 && output.effective_translation.x == 0.0 {
        commands.entity(player).remove::<Animation>();
        commands
            .entity(player)
            .insert(Animation::new(P_WALK_U, CYCLE_DELAY));
    }
}

fn anim_mov_d(
    mut commands: Commands,
    query: Query<(Entity, &KinematicCharacterControllerOutput), Without<Animation>>,
) {
    if query.is_empty() {
        return;
    }

    let (player, output) = query.single();
    if output.effective_translation.y < 0.0 && output.effective_translation.x == 0.0 {
        commands.entity(player).remove::<Animation>();
        commands
            .entity(player)
            .insert(Animation::new(P_WALK_D, CYCLE_DELAY));
    }
}

fn anim_idle(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &KinematicCharacterControllerOutput,
        &mut TextureAtlasSprite,
    )>,
) {
    if query.is_empty() {
        return;
    }

    let (player, output, mut sprite) = query.single_mut();
    if output.effective_translation.x == 0.0 {
        commands.entity(player).remove::<Animation>();
        sprite.index = P_IDLE;
    }
}

pub fn setup_player(
    mut commands: Commands,
    mut atlases: ResMut<Assets<TextureAtlas>>,
    server: Res<AssetServer>,
) {
    let image_handle: Handle<Image> = server.load("texture/player.png");
    let texture_atlas = TextureAtlas::from_grid(
        image_handle,
        Vec2::new(consts::SPRTPL_W, consts::SPRTPL_H),
        consts::SPRTPL_COLS,
        consts::SPRTPL_ROWS,
        Some(Vec2::new(31., 12.)),
        None,
    );

    let atlas_handle = atlases.add(texture_atlas);

    commands
        .spawn(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(18),
            texture_atlas: atlas_handle,
            transform: Transform {
                translation: Vec3::new(
                    consts::WINDOW_LEFT_X + 10.0,
                    consts::WINDOW_BOTTOM_Y + 30.0,
                    0.0,
                ),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RigidBody::KinematicPositionBased)
        .insert(Collider::cuboid(
            consts::SPRTPL_W / 2.,
            consts::SPRTPL_H / 2.,
        ))
        .insert(KinematicCharacterController::default());
}

pub fn movement(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut KinematicCharacterController>,
) {
    let mut player = query.single_mut();

    let mut translation = Vec2::new(0.0, 0.0);

    //move right
    if input.pressed(KeyCode::D) {
        translation.x += time.delta_seconds() * consts::PLAYER_VELOCITY_X;
    }

    //move left
    if input.pressed(KeyCode::A) {
        translation.x -= time.delta_seconds() * consts::PLAYER_VELOCITY_X;
    }

    //move up
    if input.pressed(KeyCode::W) {
        translation.y += time.delta_seconds() * consts::PLAYER_VELOCITY_X;
    }

    //move down
    if input.pressed(KeyCode::S) {
        translation.y -= time.delta_seconds() * consts::PLAYER_VELOCITY_X;
    }

    // match player.translation {
    //     Some(vec) => player.translation = Some(Vec2::new(xm,ym)),
    //     None => player.translation = Some(Vec2::new(0.0, 0.0)),
    // }
    player.translation = Some(translation);
}
