use bevy::prelude::*;

const CAMERA_MOVE_SPEED: f32 = 500.;

pub struct MoveableCameraPlugin;

#[derive(Component)]
struct MoveableCamera {
    pub velocity_x: f32,
    pub velocity_y: f32
}

impl Plugin for MoveableCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
        app.add_systems(Update, move_camera);
    }
}

fn setup_camera(
    mut commands: Commands
) {
    commands.spawn((
        Camera2d::default(),
        Transform::IDENTITY,
        MoveableCamera { 
            velocity_x: 0., 
            velocity_y: 0. 
        }
    ));
}

fn move_camera(
    mut query: Query<(&mut MoveableCamera, &mut Transform)>,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>
) {
    if let Ok((mut moveable_camera, mut transform)) = query.get_single_mut() {
        if keys.pressed(KeyCode::KeyD) {
            moveable_camera.velocity_x = CAMERA_MOVE_SPEED;
        }
        else if keys.pressed(KeyCode::KeyA) {
            moveable_camera.velocity_x = -CAMERA_MOVE_SPEED;
        }
        else if keys.pressed(KeyCode::KeyW) {
            moveable_camera.velocity_y = CAMERA_MOVE_SPEED;
        }
        else if keys.pressed(KeyCode::KeyS) {
            moveable_camera.velocity_y = -CAMERA_MOVE_SPEED;
        }
        else {
            moveable_camera.velocity_x = 0.;
            moveable_camera.velocity_y = 0.;
        }

        transform.translation.x += moveable_camera.velocity_x * time.delta_secs();
        transform.translation.y += moveable_camera.velocity_y * time.delta_secs();
    }
}