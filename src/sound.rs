use bevy::prelude::*;
//use bevy_kira_audio::{Audio,AudioPlugin, AudioSource, AudioChannel, AudioControl };

pub struct SoundPlugin;

impl Plugin for SoundPlugin{
    fn build(&self, app: &mut App) {
        app
        //.add_plugins(AudioPlugin)
        //.add_systems(PreStartup, load_sounds);
        .add_systems(schedule, systems)
    }

}


/*
#[derive(Resource)]

pub struct AudioState{
    shoot_handle : Handle<AudioSource>,
    miss_handle : Handle<AudioSource>,
    hit_handle : Handle<AudioSource>,
    volume: f32,
}

fn play_shoot_sfx(
    audio: Res<Audio>, audio_state: Res<AudioState>
)
{
    audio.play(AudioState.shoot_handle);
}

fn load_sounds(
    mut commands: Commands, 
    audio: Res<Audio>, 
    assets: Res<AssetServer>
)
{
    let shoot_handle = assets.load("audio/shoot.wav");
    let miss_handle = assets.load("audio/boom.wav");
    let hit_handle = assets.load("audio/hit.wav");
    let volume : f64 = 0.5;

    audio.set_volume(volume);

    commands.insert_resource(AudioState{
        shoot_handle : shoot_handle,
        miss_handle : miss_handle,
        hit_handle : hit_handle,
        volume : volume as f32,

    });
}
*/
/*
fn sfx_setup(
    asset_server : Res<AssetServer>,
    mut commands: Commands
)
{
    commands.spawn(AudioBundle{
        source: asset_server.load("audio/shoot.wav"),
        settings: PlaybackSettings::LOOP,
    });
}
*/