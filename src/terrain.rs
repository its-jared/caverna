use bevy::prelude::*;
use noise::{NoiseFn, Perlin};

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, generate);
    }
}

const NOISE_SCALE: f64 = 0.2;
const NOISE_THREASHOLD: f64 = 0.5;

fn generate(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    let perlin: Perlin = Perlin::new(200);

    for x in 0..50 {
        for y in 0..50 {
            let value = perlin.get([x as f64 * NOISE_SCALE, y as f64 * NOISE_SCALE]);

            if value < NOISE_THREASHOLD {
                commands.spawn((
                    Sprite {
                        image: asset_server.load("sprites/block.png"),
                        ..Default::default()
                    },
                    Transform {
                        scale: Vec3::splat(2.),
                        translation: Vec3::new(x as f32 * 16., y as f32 * 16., 0.),
                        ..Default::default()
                    },
                ));
            }
        }
    }
}
