use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use super::{GameState, Volume};

pub struct GameAudioPlugin;

#[derive(Resource)]
struct Menu;

#[derive(Resource)]
struct InGame;

#[derive(Resource)]
struct Splash;

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub enum BGMInitialized {
    Yes,
    No,
}

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub enum GMInitialized {
    Yes,
    No,
}

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AudioPlugin)
            .insert_resource(BGMInitialized::No)
            .insert_resource(GMInitialized::No)
            .add_audio_channel::<Menu>()
            .add_audio_channel::<InGame>()
            .add_audio_channel::<Splash>()
            // .add_systems(OnEnter(GameState::Menu), start_bgm)
            .add_systems(Update, start_bgm.run_if(in_state(GameState::Menu)))
            .add_systems(Update, stop_bgm.run_if(in_state(GameState::Game)))
            // .add_systems(OnEnter(GameState::Game), start_ingame)
            .add_systems(Update, start_ingame.run_if(in_state(GameState::Game)))
            .add_systems(Update, stop_ingame.run_if(in_state(GameState::Menu)))
            .add_systems(OnEnter(GameState::Splash), play_splash);
    }
}

fn play_splash(
    asset_server: Res<AssetServer>,
    audio: Res<AudioChannel<Splash>>,
    volume: Res<Volume>,
) {
    audio
        .play(asset_server.load("audios/reward.wav"))
        .with_volume(volume.get_val());
}

fn start_bgm(asset_server: Res<AssetServer>, audio: Res<AudioChannel<Menu>>, volume: Res<Volume>, mut mtus: ResMut<BGMInitialized>) {
    if *mtus == BGMInitialized::No {
        audio
            .play(asset_server.load("audios/bip-bop.ogg"))
            .with_volume(volume.get_val())
            .looped();
        audio.pause();
        *mtus = BGMInitialized::Yes;
    }
    else {
        audio.set_volume(volume.get_val());
        audio.resume();
    }
}

// fn resume_bgm(audio: Res<AudioChannel<Menu>>, volume: Res<Volume>) {
//     audio.set_volume(volume.get_val());
//     audio.resume();
// }

fn stop_bgm(audio: Res<AudioChannel<Menu>>) {
    audio.pause();
}

fn start_ingame(
    asset_server: Res<AssetServer>,
    audio: Res<AudioChannel<InGame>>,
    volume: Res<Volume>,
    mut mtus: ResMut<GMInitialized>
) {
    if *mtus == GMInitialized::No {
        audio
            .play(asset_server.load("audios/ganxta.ogg"))
            .with_volume(volume.get_val())
            .looped();
        audio.pause();
        *mtus = GMInitialized::Yes;
    }
    else {
        audio.set_volume(volume.get_val());
        audio.resume();
    }
}

// fn resume_ingame(audio: Res<AudioChannel<InGame>>, volume: Res<Volume>) {
//     audio.set_volume(volume.get_val());
//     audio.resume();
// }

fn stop_ingame(audio: Res<AudioChannel<InGame>>) {
    audio.pause();
}
