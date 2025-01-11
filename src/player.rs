use bevy::prelude::*;

fn update_player(
    time: Res<Time>,
    input_buttons: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&Player, &mut Transform)>
) {
    let mut direction = Vec3::ZERO;

    if input_buttons.pressed(KeyCode::KeyA) {
        direction.x -= 1.;
    }

    if input_buttons.pressed(KeyCode::KeyD) {
        direction.x += 1.;
    }

    if input_buttons.pressed(KeyCode::KeyW) {
        direction.y += 1.;
    }

    if input_buttons.pressed(KeyCode::KeyS) {
        direction.y -= 1.;
    }

    if direction != Vec3::ZERO {
        let speed = 100.;
        let translation = direction.normalize() * speed * time.delta_secs();
        
        player_query.iter_mut().for_each(|(_player, mut player_translation)| {
            player_translation.translation += translation;
        });
    }
}

fn init_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn(Camera2d::default());

    commands.spawn((
        Transform::from_scale(Vec3::splat(2.)),
        Sprite {
            image: asset_server.load("sprites/player.png"),
            ..Default::default()
        },
        Player {
            health: 100
        }
    ));
}

#[derive(Component)]
pub struct Player {
    pub health: u32
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_player);
        app.add_systems(Update, update_player);
    }
}