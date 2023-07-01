use bevy::prelude::*;

use crate::assets::AudioAssets;
use crate::GameState;

pub struct MusicPlugin;

impl Plugin for MusicPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(play_music.in_schedule(OnEnter(GameState::Playing)))
        ;
    }
}

fn play_music(audio_assets: Res<AudioAssets>, audio: Res<Audio>) {
    audio.play_with_settings(audio_assets.music_1.clone(), PlaybackSettings::LOOP.with_volume(0.3));
}
