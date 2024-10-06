use bevy::prelude::*;

use crate::rat;
use crate::resolution;
use crate::player;
use crate::projectile;
use crate::audio;
pub struct GamePlugin;

impl Plugin for GamePlugin{
    fn build(&self, app: &mut App){
        app
        .add_plugins(
            (
                rat::RatPlugin,
                resolution::ResolutionPlugin,
                player::PlayerPlugin,
                projectile::ProjectilePlugin,
                audio::AudioPlugin
            )
        )
        .add_systems(Startup, setup_scene);
    }
}
fn setup_scene(mut commands : Commands)
{
    commands.spawn(Camera2dBundle{..default()});
}