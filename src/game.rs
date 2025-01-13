use bevy::prelude::*;

use crate::gui::hud::HudPlugin;
use crate::player::PlayerPlugin;
use crate::level::LevelPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            PlayerPlugin,
            LevelPlugin,
            HudPlugin
        ));
    }
}