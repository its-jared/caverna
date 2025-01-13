use bevy::prelude::*;
use bevy_rapier2d::prelude::Collider;
use noise::NoiseFn;

pub const WORLD_SIZE: i32 = 50;
pub const TRUE_WORLD_SIZE: i32 = WORLD_SIZE * 16;

fn init_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let perlin = noise::Perlin::new(1);

    for x in 0..WORLD_SIZE {
        for y in 0..WORLD_SIZE {
            let noise = perlin.get([x as f64 * 0.2, y as f64 * 0.2]);

            if noise > 0.5 {
                commands.spawn((
                    Transform {
                        translation: Vec3::new(x as f32 * 16., y as f32 * 16., 0.),
                        scale: Vec3::splat(2.),
                        ..Default::default()
                    },
                    Sprite {
                        image: asset_server.load("sprites/bricks.png"),
                        ..Default::default()
                    },
                    Collider::cuboid(8. / 2., 8. / 2.)
                ));   
            }
        }
    }
}

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_level);
    }
}