use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

fn update_player(
    time: Res<Time>,
    input_buttons: Res<ButtonInput<KeyCode>>,
    mut controllers: Query<&mut KinematicCharacterController>,
    mut cameras: Query<(&mut Camera2d, &mut Transform)>
) {
    let mut direction = Vec2::ZERO;

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

    if direction != Vec2::ZERO {
        let speed = 100.;
        let translation = direction.normalize() * speed * time.delta_secs();
         
        for mut controller in controllers.iter_mut() {
            controller.translation = Some(translation);
        }

        for (mut _camera, mut transform) in cameras.iter_mut() {
            transform.translation += Vec3::new(translation.x, translation.y, 0.);
        }
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
        },
        RigidBody::KinematicPositionBased,
        Collider::ball(8. / 2.),
        KinematicCharacterController::default()
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