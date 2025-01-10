use bevy::prelude::*;

mod game;
mod moveable_camera;
mod terrain;

fn main() {
    App::new()
        .add_plugins(
            (
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
            )
        )
        .run();
}

