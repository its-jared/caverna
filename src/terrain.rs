use bevy::prelude::*;
use rand::prelude::*;
use noise::{NoiseFn, Perlin};

use crate::block::*;

const NOISE_SCALE: f64 = 0.25;
const NOISE_THREASHOLD: f64 = 0.6;

const WORLD_SIZE: i32 = 500;

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_terrain);
    }
}

fn init_terrain(
    commands: Commands,
    asset_server: Res<AssetServer>
) {
    let blocks: Vec<BlockComponent> = init_blocks(asset_server);
    generate(commands, blocks);
}

fn generate(
    mut commands: Commands,
    blocks: Vec<BlockComponent>
) {
    let perlin: Perlin = Perlin::new(200);
    let mut rng: ThreadRng = thread_rng();

    for x in 0..WORLD_SIZE {
        for y in 0..WORLD_SIZE {
            let value = perlin.get([x as f64 * NOISE_SCALE, y as f64 * NOISE_SCALE]);
            let mut block_id = 0;

            if value < NOISE_THREASHOLD {
                if rng.gen_range(0..10) >= 5 {
                    block_id = 2;
                } else {
                    block_id = 1;
                }
            }

            
            let block: &BlockComponent = blocks.get(block_id).unwrap();
            
            commands.spawn((
                Sprite {
                    image: block.sprite.clone(),
                    ..Default::default()
                },
                Transform {
                    scale: Vec3::splat(BLOCK_SCALE),
                    translation: Vec3::new(x as f32 * (BLOCK_SCALE * BLOCK_PIXEL_SIZE), 
                                        y as f32 * (BLOCK_SCALE * BLOCK_PIXEL_SIZE),
                                        0.),
                    ..Default::default()
                },
                BlockComponent {
                    id: block.id,
                    name: block.name.clone(),
                    block_description: block.block_description.clone(),
                    sprite: block.sprite.clone()
                }
            ));
        }
    }
}