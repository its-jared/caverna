use bevy::prelude::*;

use crate::player::PlayerPlugin;
use crate::level::LevelPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            PlayerPlugin,
            LevelPlugin
        ));
    }
}