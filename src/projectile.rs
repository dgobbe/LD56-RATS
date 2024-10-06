use bevy::{ecs::entity, prelude::*};

use crate::{rat, resolution};
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
    mut projectile_query : Query<(Entity, &Projectile, &mut Transform)>,
    time : Res<Time>,
    resolution : Res<resolution::Resolution>
)
{
    for (entity, projectile, mut transform) in projectile_query.iter_mut()
    {
        transform.translation.y += projectile.speed * time.delta_seconds();
        if transform.translation.y > resolution.screen_dimensions.y * 0.5
        {
            commands.entity(entity).despawn();
        }
    }
}


fn update_rat_hits(
    mut rat_query : Query<(&mut rat::Rat, &Transform), Without<rat::Dead>>,
    mut projectile_query : Query<(Entity, &Transform), With<Projectile>>,
    mut commands: Commands
)
{
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
                rat.dead = true;
                commands.entity(projectile_entity).despawn();
            }
        }
    }
}