use bevy::prelude::*;
use crate::moveable_camera::*;
use crate::terrain::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            (
                MoveableCameraPlugin,
                TerrainPlugin,
            )
        );
    }
}