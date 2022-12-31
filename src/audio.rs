use std::collections::HashMap;

use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl, AudioPlugin, AudioSource};
use strum::{EnumIter, IntoStaticStr};

// region: Plugin Setup
pub struct InternalAudioPlugin;

impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AudioPlugin)
            .add_startup_system_to_stage(StartupStage::PreStartup, load_music)
            .add_startup_system_to_stage(StartupStage::PostStartup, start_music);
    }
}
// endregion

// region: Stuctures

#[derive(EnumIter, IntoStaticStr, PartialEq, Eq, Hash, Clone, Copy)]
pub enum AudioTrack {
    MainTheme,
}

#[derive(Resource)]
pub struct AudioResources {
    pub audio: HashMap<AudioTrack, Handle<AudioSource>>,
}

// endregion

// region: Systems

fn load_music(mut commands: Commands, assets: Res<AssetServer>) {
    let song: Handle<AudioSource> = assets.load("sounds\\Main Theme.mp3");
    commands.insert_resource(AudioResources {
        audio: HashMap::from([(AudioTrack::MainTheme, song)]),
    })
}

fn start_music(audio: Res<Audio>, audio_resources: Res<AudioResources>) {
    let Some(main_theme) = audio_resources
        .audio
        .get(&AudioTrack::MainTheme) else { return };

    audio.play(main_theme.clone()).looped();
}

// endregion
