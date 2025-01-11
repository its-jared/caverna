use bevy::prelude::*;

fn init_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn((
        Transform {
            translation: Vec3::new(10. * 16., 0., 0.),
            scale: Vec3::splat(2.),
            ..Default::default()
        },
        Sprite {
            image: asset_server.load("sprites/bricks.png"),
            ..Default::default()
        }
    ));
}

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_level);
    }
}