use bevy::prelude::*;
use bevy_audio::Volume;

use crate::audio::Sound;
use crate::resolution;
use crate::projectile;
use crate::audio;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin{
    fn build(&self, app : &mut App)
    {
        app.add_systems(Startup, setup_player)
        .add_systems(Update, update_player);
    }
}

#[derive(Component)]
struct Player{
    pub shoot_timer : f32,
}

fn setup_player(
    mut commands : Commands,
    asset_server : Res<AssetServer>,
    resolution : Res<resolution::Resolution>, 
)
{
    let player_image = asset_server.load("sprites/ship.png");
    commands.spawn((
        SpriteBundle{
            texture: player_image,
            transform: Transform::from_xyz(0.,-(resolution.screen_dimensions.y*0.5) + (resolution.pixel_ratio*5.0), 0.).with_scale(Vec3::splat(resolution.pixel_ratio)),
            ..Default::default()
        },
        Player{shoot_timer : 0.}
    ));

}
const SPEED : f32 = 200.;
const BULLET_SPEED : f32 = 400.;
const SHOOT_COOLDOWN : f32 = 0.1;
fn update_player(
    mut commands : Commands,
    asset_server : Res<AssetServer>,
    mut player_query : Query<(&mut Player, &mut Transform)>,
    time : Res<Time>,
    keys : Res<ButtonInput<KeyCode>>,
    resolution : Res<resolution::Resolution>,
    sound : Res<audio::Sound>
)
{
    let(mut player, mut transform) = player_query.single_mut();

    let mut horizontal : f32 = 0.;
    let mut vertical : f32 = 0.;

    if keys.pressed(KeyCode::KeyA){
        horizontal += -1.;
    }
    if keys.pressed(KeyCode::KeyD){
        horizontal += 1.;
    }
    if keys.pressed(KeyCode::KeyS){
        vertical += -1.;
    }
    if keys.pressed(KeyCode::KeyW){
        vertical += 1.;
    }

    transform.translation.x += horizontal * time.delta_seconds() * SPEED;
    transform.translation.y += vertical * time.delta_seconds() * SPEED;

    let left_bound = -resolution.screen_dimensions.x * 0.5;
    let right_bound = resolution.screen_dimensions.x * 0.5;

    if transform.translation.x > right_bound
    {
        transform.translation.x = right_bound;
    }
    if transform.translation.x < left_bound 
    {
        transform.translation.x = left_bound;
    }
    player.shoot_timer -= time.delta_seconds();

    if keys.pressed(KeyCode::Space)&&player.shoot_timer <= 0.
    {
        player.shoot_timer = SHOOT_COOLDOWN;
        let bullet_texture = asset_server.load("sprites/bullet.png");
        commands.spawn((
            SpriteBundle{
                texture : bullet_texture,
                transform : Transform::from_translation(transform.translation).with_scale(Vec3::splat(resolution.pixel_ratio)),
                ..Default::default()
            },
            projectile::Projectile{
                speed : BULLET_SPEED
            }
        ));
        play_shoot_sfx(commands, asset_server, sound);
    }

}
fn play_shoot_sfx(
    mut commands : Commands, 
    asset_server : Res<AssetServer>, 
    sound : Res<audio::Sound>
)
{
    commands.spawn(AudioBundle{
        source : asset_server.load("audio/shoot.wav"),
        settings : PlaybackSettings{
            volume : sound.volume,
            ..default()
        },
        ..default()
    });
}