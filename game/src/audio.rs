use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use super::{GameState, Volume};

pub struct GameAudioPlugin;

#[derive(Resource)]
struct Menu;

#[derive(Resource)]
struct InGame;

impl Plugin for GameAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AudioPlugin)
            .add_audio_channel::<Menu>()
            .add_audio_channel::<InGame>()
            .add_systems(OnEnter(GameState::Menu), start_bgm)
            .add_systems(Update, resume_bgm.run_if(in_state(GameState::Menu)))
            .add_systems(Update, pause_bgm.run_if(in_state(GameState::Game)))
            
            .add_systems(OnEnter(GameState::Game), start_ingame)
            .add_systems(Update, resume_ingame.run_if(in_state(GameState::Game)))
            .add_systems(Update, stop_ingame.run_if(in_state(GameState::Menu)));
    }
}

// pub fn play_reward_sfx(audio: Res<Audio>, audio_state: Res<AudioState>) {
//     audio.play_in_channel(audio_state.reward_handle.clone(), &audio_state.sfx_channel);
// }

// fn play_hit_sfx(
//     audio: Res<Audio>,
//     audio_state: Res<AudioState>,
//     mut fight_event: EventReader<FightEvent>,
// ) {
//     if fight_event.iter().count() > 0 {
//         audio.play_in_channel(audio_state.hit_handle.clone(), &audio_state.sfx_channel);
//     }
// }

// fn resume_bgm_music(audio: Res<Audio>, audio_state: Res<AudioState>) {
//     audio.stop_channel(&audio_state.combat_channel);
//     audio.resume_channel(&audio_state.bgm_channel);
// }

// fn start_combat_music(audio: Res<Audio>, audio_state: Res<AudioState>) {
//     audio.pause_channel(&audio_state.bgm_channel);
//     audio.play_looped_in_channel(
//         audio_state.combat_handle.clone(),
//         &audio_state.combat_channel,
//     );
// }

// fn volume_control(
//     keyboard: Res<Input<KeyCode>>,
//     audio: Res<Audio>,
//     mut audio_state: ResMut<AudioState>,
// ) {
//     if keyboard.just_pressed(KeyCode::Up)  &&{
//         audio_state.volume += 0.10;
//     }
//     if keyboard.just_pressed(KeyCode::Down) {
//         audio_state.volume -= 0.10;
//     }
//     audio_state.volume = audio_state.volume.clamp(0.0, 1.0);
//     audio.set_volume_in_channel(audio_state.volume, &audio_state.bgm_channel);
// }

// fn start_bgm_music(audio: Res<Audio>, audio_state: Res<AudioState>) {
//     audio.play_looped_in_channel(audio_state.bgm_handle.clone(), &audio_state.bgm_channel);
// }

fn start_bgm(asset_server: Res<AssetServer>, audio: Res<AudioChannel<Menu>>, volume: Res<Volume>) {
    audio.play(asset_server.load("audios/bip-bop.ogg")).with_volume(volume.get_val());
    audio.pause();
}

fn resume_bgm(audio: Res<AudioChannel<Menu>>, volume: Res<Volume>) {
    audio.set_volume(volume.get_val());
    audio.resume();
}

fn pause_bgm(audio: Res<AudioChannel<Menu>>) {
    audio.pause();
}

fn start_ingame(asset_server: Res<AssetServer>, audio: Res<AudioChannel<InGame>>, volume: Res<Volume>) {
    audio.play(asset_server.load("audios/ganxta.ogg")).with_volume(volume.get_val());
    audio.pause();
}

fn resume_ingame(audio: Res<AudioChannel<InGame>>, volume: Res<Volume>) {
    audio.set_volume(volume.get_val());
    audio.resume();
}

fn stop_ingame(audio: Res<AudioChannel<InGame>>) {
    audio.stop();
}

// fn load_audio(mut commands: Commands, audio: Res<Audio>, assets: Res<AssetServer>, setting_volume: Res<Volume>) {
//     let bgm_handle = assets.load("audios/bip-bop.ogg");
//     let combat_handle = assets.load("audios/ganxta.ogg");
//     let hit_handle = assets.load("audios/hit.wav");
//     let reward_handle = assets.load("audios/reward.wav");

//     let bgm_channel = AudioChannel::new("bgm".to_string());
//     let combat_channel = AudioChannel::new("combat".to_string());
//     let sfx_channel = AudioChannel::new("sfx".to_string());
//     let volume = setting_volume / 10.;

//     audio.set_volume_in_channel(volume, &bgm_channel);
//     audio.set_volume_in_channel(volume, &combat_channel);
//     audio.set_volume_in_channel(volume, &sfx_channel);

//     commands.insert_resource(AudioState {
//         bgm_handle: bgm_handle,
//         combat_handle: combat_handle,
//         hit_handle: hit_handle,
//         reward_handle: reward_handle,
//         bgm_channel,
//         combat_channel,
//         sfx_channel,
//         volume,
//     });
// }