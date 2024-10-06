use bevy::{ecs::entity, prelude::*};

use crate::{rat, resolution};
use crate::audio;
pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin{
    fn build(&self, app: &mut App){
        app.add_systems(Update, (update_projectiles, update_rat_hits));
    }
}

const BULLET_RADIUS : f32 = 24.;

#[derive(Component)]
pub struct Projectile{
    pub speed : f32,
} 

fn update_projectiles(
    mut commands: Commands,
    asset_server : Res<AssetServer>,
    mut projectile_query : Query<(Entity, &Projectile, &mut Transform)>,
    time : Res<Time>,
    resolution : Res<resolution::Resolution>,
    sound : Res<audio::Sound>
)
{
    let mut missed : bool = false;
    for (entity, projectile, mut transform) in projectile_query.iter_mut()
    {
        transform.translation.y += projectile.speed * time.delta_seconds();
        if transform.translation.y > resolution.screen_dimensions.y * 0.5
        {
            missed = true;  
            commands.entity(entity).despawn();
        }
    }
    if missed {
        play_hit_sfx(false, commands, &asset_server, sound); 
    }
}


fn update_rat_hits(
    asset_server : Res<AssetServer>,
    mut rat_query : Query<(&mut rat::Rat, &Transform), Without<rat::Dead>>,
    mut projectile_query : Query<(Entity, &Transform), With<Projectile>>,
    mut commands: Commands,
    sound : Res<audio::Sound>
    
)
{
    let mut hit : bool = false;
    for(mut rat, rat_transform) in rat_query.iter_mut()
    {
        for(projectile_entity, projectile_transform) in projectile_query.iter_mut(){
            let projectile_pos = Vec2::new(
                projectile_transform.translation.x,
                projectile_transform.translation.y,
            );
            let rat_pos = Vec2::new(
                rat_transform.translation.x, 
                rat_transform.translation.y
            );
            if Vec2::distance(rat_pos, projectile_pos) < BULLET_RADIUS
            {
                hit = true;
                rat.dead = true;
                commands.entity(projectile_entity).despawn();
            }
        }
    }
    if hit {
        play_hit_sfx(true, commands, &asset_server, sound);
    }
}

fn play_hit_sfx(
    hit : bool,
    mut commands : Commands, 
    asset_server : &Res<AssetServer>,
    sound : Res<audio::Sound>
)
{
    let mut path : &str;
    if hit {
        path = "audio/boom.wav";
    }else{
        path = "audio/hit.wav";
    }
    commands.spawn(AudioBundle{
        source : asset_server.load(path),
        settings : PlaybackSettings{
            volume : sound.volume,
            ..default()
        },
        ..default()
    });
}