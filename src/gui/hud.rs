use bevy::diagnostic::{self, DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use::bevy::prelude::*;

#[derive(Component)]
struct FpsText;

fn setup_hud(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands
        .spawn(Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            justify_content: JustifyContent::SpaceBetween,
            ..default()
        })
        .insert(PickingBehavior::IGNORE)
        .with_children(|parent| {
            parent
                .spawn((
                    FpsText,
                    Text::new("FPS: "),
                    TextFont {
                        font: asset_server.load("font.ttf"),
                        font_size: 25.,
                        ..default()
                    },
                ));
        });
}

fn update_fps_text(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>
) {
    for mut text in &mut query {
        if let Some(value) = diagnostics
            .get(&FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|fps| fps.smoothed()) {
            }
    }
}

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_hud);
    }
}