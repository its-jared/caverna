use bevy::prelude::*;

pub mod game;

pub mod player;
pub mod level;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: String::from("caverna"),
                    position: WindowPosition::Centered(MonitorSelection::Primary),
                    resizable: false,
                    ..Default::default()
                }),
                ..Default::default()
            })
            .set(ImagePlugin::default_nearest()),
            game::GamePlugin
        ))
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .run();
}