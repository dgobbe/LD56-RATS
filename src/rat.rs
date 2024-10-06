use bevy::{asset::transformer, prelude::*, render::view::visibility};

use crate::resolution;

pub struct RatPlugin;

impl Plugin for RatPlugin{
    fn build(&self, app: &mut App){
        app
        .add_systems(Startup, setup_rats)
        .add_systems(Update, (update_rats,manage_rat_logic));
    }
}
#[derive(Component)]
pub struct Rat{
    pub dead : bool,
    pub original_position : Vec3,
}
#[derive(Component)]
pub struct Dead; 

#[derive(Resource)]
pub struct RatManager{
    pub direction : f32,
    pub shift_rats_down : bool,
    pub dist_from_boundary : f32,
    pub reset : bool,
} 

const WIDTH : i32 = 10;
const HEIGHT : i32 = 5;
const SPACING : f32 = 24.;
const SPEED : f32 = 100.;
const  RAT_SHIFT_AMOUNT : f32 = 32.; 
//spawn the rats
fn setup_rats(mut commands: Commands, asset_server : Res<AssetServer>, resolution : Res<resolution::Resolution>)
{
    commands.insert_resource(RatManager{
        reset : false,
        dist_from_boundary : 0.,
        shift_rats_down : false,
        direction : 1.,
    });
    let rat_texture = asset_server.load("sprites/rat.png");
    for x in 0..WIDTH
    {
        for y in 0..HEIGHT
        {
            let position = Vec3::new(x as f32 * SPACING, y as f32 * SPACING, 0.)
                -(Vec3::X*WIDTH as f32 * SPACING * 0.5)
                -(Vec3::Y*HEIGHT as f32 * SPACING * 1.0)
                +(Vec3::Y * resolution.screen_dimensions.y * 0.5);
            commands.spawn((    
                SpriteBundle{
                    transform : Transform::from_translation(position).with_scale(Vec3::splat(resolution.pixel_ratio)),
                    texture : rat_texture.clone(),
                    ..default()
                },
                Rat{
                    original_position : position,
                    dead : false,

                }
            ));
        }
    }
    
}

fn update_rats(
    mut commands: Commands,
    mut rat_query : Query<(Entity, &Rat, &mut Transform, &mut Visibility),Without<Dead>>,
    mut rat_manager : ResMut<RatManager>,
    resolution : Res<resolution::Resolution>,
    time : Res<Time>,
)
{
    for(entity, rat, mut transform, mut visibility) in rat_query.iter_mut()
    {
        transform.translation.x += time.delta_seconds() * rat_manager.direction * SPEED;
        if transform.translation.x.abs() > resolution.screen_dimensions.x * 0.5 {
            rat_manager.shift_rats_down = true;
            rat_manager.dist_from_boundary = resolution.screen_dimensions.x * rat_manager.direction * 0.5 - transform.translation.x;
        }
        if rat.dead{
            commands.entity(entity).insert(Dead{});
            *visibility = Visibility::Hidden;
        }
        else{
            *visibility = Visibility::Visible;
        }
        if transform.translation.y < -resolution.screen_dimensions.y*0.5{
            rat_manager.reset = true;
        }
    }
}

fn manage_rat_logic(
    mut commands: Commands,
    mut rat_query : Query<(Entity, &mut Rat, &mut Transform)>, 
    mut rat_manager : ResMut<RatManager>
)
{
    if rat_manager.shift_rats_down {
        rat_manager.shift_rats_down = false;
        rat_manager.direction *= -1.;
        for(_entity, _rat, mut transform) in rat_query.iter_mut()
        {
            transform.translation.x += rat_manager.dist_from_boundary;
            transform.translation.y -= RAT_SHIFT_AMOUNT;
        }
    }
    if rat_manager.reset{
        rat_manager.reset = false;
        rat_manager.direction = 1.;
        for(entity, mut rat, mut transform) in rat_query.iter_mut()
        {
            transform.translation = rat.original_position;
            if rat.dead{
                rat.dead = false;
                commands.entity(entity).remove::<Dead>();
            }
        }
    }
}