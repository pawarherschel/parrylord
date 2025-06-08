use crate::assets::{GameplayMusic, MusicAudio, NotGameplayMusic};
use crate::AudioSpawned;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.register_type::<Music>();
    app.register_type::<SoundEffect>();

    app.add_systems(
        Update,
        apply_global_volume.run_if(resource_changed::<GlobalVolume>),
    );
}

/// An organizational marker component that should be added to a spawned [`AudioPlayer`] if it's in the
/// general "music" category (e.g. global background music, soundtrack).
///
/// This can then be used to query for and operate on sounds in that category.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Music;

/// A music audio instance.
pub fn music(handle: Handle<AudioSource>) -> impl Bundle {
    (AudioPlayer(handle), PlaybackSettings::LOOP, Music)
}

/// An organizational marker component that should be added to a spawned [`AudioPlayer`] if it's in the
/// general "sound effect" category (e.g. footsteps, the sound of a magic spell, a door opening).
///
/// This can then be used to query for and operate on sounds in that category.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct SoundEffect;

/// A sound effect audio instance.
pub fn sound_effect(handle: Handle<AudioSource>) -> impl Bundle {
    (AudioPlayer(handle), PlaybackSettings::DESPAWN, SoundEffect)
}

/// [`GlobalVolume`] doesn't apply to already-running audio entities, so this system will update them.
fn apply_global_volume(
    global_volume: Res<GlobalVolume>,
    mut audio_query: Query<(&PlaybackSettings, &mut AudioSink)>,
) {
    for (playback, mut sink) in &mut audio_query {
        sink.set_volume(global_volume.volume * playback.volume);
    }
}

pub fn spawn_music(
    mut commands: Commands,
    music_audio: Option<Res<MusicAudio>>,
    mut flag: ResMut<AudioSpawned>,
) {
    let Some(music_audio) = music_audio else {
        return;
    };

    info!("spawning music");

    flag.0 = true;

    commands.spawn((music(music_audio.gameplay.clone()), GameplayMusic));
    commands.spawn((music(music_audio.not_gameplay.clone()), NotGameplayMusic));
}

pub fn pause_gameplay_music(sink: Single<&AudioSink, With<GameplayMusic>>) {
    sink.pause();
}

pub fn resume_gameplay_music(sink: Single<&AudioSink, With<GameplayMusic>>) {
    sink.play();
}

pub fn pause_not_gameplay_music(sink: Single<&AudioSink, With<NotGameplayMusic>>) {
    sink.pause();
}

pub fn resume_not_gameplay_music(sink: Single<&AudioSink, With<NotGameplayMusic>>) {
    sink.play();
}
