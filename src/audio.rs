use bevy::prelude::*;
use bevy_audio::Volume;

pub struct AudioPlugin;

impl Plugin for AudioPlugin{
    fn build(&self, app: &mut App){
        app.add_systems(PreStartup, setup_audio);
    }
}

#[derive(Resource)]
pub struct Sound{
    pub volume : Volume,
} 
fn setup_audio(mut commands : Commands)
{

    commands.insert_resource(Sound{
        volume : Volume::new(0.1),
    });
}