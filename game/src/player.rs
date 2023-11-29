use bevy::{
    prelude::*,
    text::{BreakLineOn, Text2dBounds},
};
use bevy_rapier2d::prelude::*;

use std::time::Duration;

use crate::consts;
use game::animator::{Animation, AnimationPlugin};

use super::{
    despawn_screen, DisplayQuality, GameState, GameTimer, OnGameScreen, PlayerFlag, Volume,
    WinFlag, TEXT_COLOR,
};

const P_WALK_U: &[usize] = &[0, 1, 2, 3, 4, 5, 6, 7, 8];
const P_WALK_L: &[usize] = &[9, 10, 11, 12, 13, 14, 15, 16, 17];
const P_WALK_D: &[usize] = &[18, 19, 20, 21, 22, 23, 24, 25, 26];
const P_WALK_R: &[usize] = &[27, 28, 29, 30, 31, 32, 33, 34, 35];
const P_IDLE: usize = 18;

const CYCLE_DELAY: Duration = Duration::from_millis(100);

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub enum PlayerStatus {
    Active,
    Paused,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), setup_player)
            .insert_resource(PlayerStatus::Active)
            .add_systems(Update, movement.run_if(in_state(GameState::Game)))
            .add_systems(Update, anim_idle.run_if(in_state(GameState::Game)))
            .add_systems(Update, anim_mov_d.run_if(in_state(GameState::Game)))
            .add_systems(Update, anim_mov_l.run_if(in_state(GameState::Game)))
            .add_systems(Update, anim_mov_r.run_if(in_state(GameState::Game)))
            .add_systems(Update, anim_mov_u.run_if(in_state(GameState::Game)))
            .add_systems(Update, display_events.run_if(in_state(GameState::Game)))
            .insert_resource(GameTimer(Timer::from_seconds(3.0, TimerMode::Once)))
            .add_systems(Update, exit_game.run_if(in_state(GameState::Game)))
            .add_systems(Update, hot_keys.run_if(in_state(GameState::Game)))
            .add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>)
            .add_plugins(AnimationPlugin);
        // app.add_systems(Startup, setup_player)
        //     .add_systems(Update, movement)
        //     .add_systems(Update, anim_idle)
        //     .add_systems(Update, anim_mov_d)
        //     .add_systems(Update, anim_mov_u)
        //     .add_systems(Update, anim_mov_r)
        //     .add_systems(Update, anim_mov_l)
        //     .add_plugins(AnimationPlugin);
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
        .spawn((
            SpriteSheetBundle {
                sprite: TextureAtlasSprite::new(18),
                texture_atlas: atlas_handle,
                transform: Transform {
                    translation: Vec3::new(
                        consts::WINDOW_LEFT_X + 730.0,
                        consts::WINDOW_BOTTOM_Y + 50.0,
                        0.0,
                    ),
                    ..Default::default()
                },
                ..Default::default()
            },
            OnGameScreen,
            PlayerFlag,
        ))
        .insert(RigidBody::Dynamic)
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Velocity {
            linvel: Vec2::new(1., 0.),
            angvel: 0.,
        })
        .insert(Collider::cuboid(
            consts::SPRTPL_W / 2.,
            consts::SPRTPL_H / 2.,
        ))
        .insert(KinematicCharacterController::default());
}

fn display_events(
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
    query: Query<Entity, With<WinFlag>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut game_state: ResMut<NextState<GameState>>,
    mut pstatus: ResMut<PlayerStatus>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(entity1, entity2, _flags) => {
                if let Ok(component) = query.get(*entity1) {
                    println!("Started: Entity1's component = {:?}", component);

                    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
                    let text_style = TextStyle {
                        font: font.clone(),
                        font_size: 100.0,
                        color: Color::WHITE,
                    };
                    let text_alignment = TextAlignment::Center;
                    // Demonstrate changing translation
                    commands.spawn((
                        Text2dBundle {
                            text: Text::from_section("You WIN!", text_style.clone())
                                .with_alignment(text_alignment),
                            ..default()
                        },
                        OnGameScreen,
                    ));

                    *pstatus = PlayerStatus::Paused;
                    commands.insert_resource(GameTimer(Timer::from_seconds(3.0, TimerMode::Once)));
                }
                if let Ok(component) = query.get(*entity2) {
                    println!("Started: Entity2's component = {:?}", component);

                    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
                    let text_style = TextStyle {
                        font: font.clone(),
                        font_size: 100.0,
                        color: Color::WHITE,
                    };
                    let text_alignment = TextAlignment::Center;
                    // Demonstrate changing translation
                    commands.spawn((
                        Text2dBundle {
                            text: Text::from_section("You WIN!", text_style.clone())
                                .with_alignment(text_alignment),
                            ..default()
                        },
                        OnGameScreen,
                    ));

                    *pstatus = PlayerStatus::Paused;
                    commands.insert_resource(GameTimer(Timer::from_seconds(3.0, TimerMode::Once)));
                }
            }
            CollisionEvent::Stopped(entity1, entity2, _flags) => {
                println!("Stopped: Entity1 = {:?}, Entity2 = {:?}", entity1, entity2);
            }
        }
    }

    for contact_force_event in contact_force_events.iter() {
        println!("Received contact force event: {:?}", contact_force_event);
    }
}

fn exit_game(
    time: Res<Time>,
    mut game_state: ResMut<NextState<GameState>>,
    mut timer: ResMut<GameTimer>,
    mut pstatus: ResMut<PlayerStatus>,
) {
    if *pstatus == PlayerStatus::Paused && timer.tick(time.delta()).finished() {
        game_state.set(GameState::Menu);
    }
}

fn hot_keys(input: Res<Input<KeyCode>>, mut game_state: ResMut<NextState<GameState>>) {
    if input.just_pressed(KeyCode::Escape) {
        game_state.set(GameState::Menu);
    }
}

pub fn movement(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut KinematicCharacterController>,
    pstatus: Res<PlayerStatus>,
) {
    let mut player = query.single_mut();

    let mut translation = Vec2::new(0.0, 0.0);

    // let is_active = match pstatus {
    //     Res<State<PlayerStatus>>::Active => true,
    //     Res<State<PlayerStatus>>::Paused => false
    // };

    //move right
    if *pstatus == PlayerStatus::Active {
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
    if output.effective_translation.x == 0.0 && output.effective_translation.y == 0.0 {
        commands.entity(player).remove::<Animation>();
        sprite.index = P_IDLE;
    }
}
