use bevy::prelude::*;

pub mod game;
pub mod rat;
pub mod resolution;
pub mod player;
pub mod projectile;

fn main()
{
    App::new()
        .add_plugins(
            (
                DefaultPlugins
                    .set(WindowPlugin{
                        primary_window : Some(Window{
                            title : String::from("Cool game :)"),
                            position : WindowPosition::Centered(MonitorSelection::Primary),
                            resolution : Vec2::new(512., 512.).into(),
                            ..Default::default()
                        })
                        ,..Default::default()
                    })
                    .set(ImagePlugin::default_nearest()),

                    game::GamePlugin,
                ),
        )
        .run();
}