use bevy::prelude::*;
use bevy_audio::{AudioSource, AudioPlugin, PlaybackSettings, Volume};

#[derive(Resource)]
pub struct BackgroundMusic(Handle<AudioSource>);

#[derive(Resource)]
pub struct MistakeSound(Handle<AudioSource>);

pub fn play_background_music(
    audio: Res<BackgroundMusic>,
    audio_source: Res<Assets<AudioSource>>,
    mut commands: Commands,
) {
    commands.spawn(AudioSourceBundle {
        source: audio.0.clone(),
        settings: PlaybackSettings {
            volume: Volume::new(0.03),
            ..Default::default()
        },
        ..Default::default()
    });
}

pub fn insert_audio_resource(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let background_music = asset_server.load("audio/background.ogg");
    commands.insert_resource(BackgroundMusic(background_music));
    let mistake_sound = asset_server.load("audio/mistake.ogg");
    commands.insert_resource(MistakeSound(mistake_sound));
}

pub fn play_mistake_sound(
    audio: Res<MistakeSound>,
    audio_source: Res<Assets<AudioSource>>,
    commands: &mut Commands,
) {
    commands.spawn(AudioSourceBundle {
        source: audio.0.clone(),
        settings: PlaybackSettings {
            volume: Volume::new(0.03),
            ..Default::default()
        },
        ..Default::default()
    });
}
