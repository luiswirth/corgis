use amethyst::{
    assets::Handle,
    core::{math::Vector3, Time, Transform},
    ecs::{prelude::*, Component, DenseVecStorage, Entity, World},
    prelude::{Builder, WorldExt},
    renderer::{
        palette::{Hsl, RgbHue, Srgba},
        resources::Tint,
        SpriteRender, SpriteSheet,
    },
};

use super::Universe;

pub struct TileEntities(pub Vec<Entity>);

impl TileEntities {
    pub fn get_at_pos(&self, x: f32, y: f32) -> Option<&Entity> {
        let (idx, idy) = Tile::get_indices(x, y);
        let tile_index = idy * Tile::MAP_WIDTH + idx;
        self.0.get(tile_index as usize)
    }
}

#[derive(Clone)]
pub struct Tile {
    pub ttype: TileType,
}

impl Tile {
    pub const SPRITE_SIZE: f32 = 4.0; // TODO: query this automatically
    pub const SIZE: f32 = 20.0;
    pub const MAP_WIDTH: u32 = Universe::WIDTH_TILE;
    pub const MAP_HEIGHT: u32 = Universe::HEIGHT_TILE;

    pub fn get_pos(idx: u32, idy: u32) -> (f32, f32) {
        (
            idx as f32 * Tile::SIZE + Tile::SIZE / 2.0,
            idy as f32 * Tile::SIZE + Tile::SIZE / 2.0,
        )
    }

    pub fn get_indices(pos_x: f32, pos_y: f32) -> (u32, u32) {
        ((pos_x / Tile::SIZE) as u32, (pos_y / Tile::SIZE) as u32)
    }
}

impl Default for Tile {
    fn default() -> Self {
        Tile {
            ttype: TileType::default(),
        }
    }
}

impl Component for Tile {
    type Storage = DenseVecStorage<Tile>;
}

#[derive(Debug, Clone, PartialEq)]
pub enum TileType {
    Neutral,
    Blue,
    Red,
}

impl Default for TileType {
    fn default() -> Self {
        Self::Neutral
    }
}

pub fn create_tiles(world: &mut World) {
    //world.register::<Tile>();
    let sprite_render = {
        let sprite_sheet = world.fetch::<Handle<SpriteSheet>>();
        SpriteRender::new((*sprite_sheet).clone(), 1)
    };
    let tint = Tint(Srgba::new(1.0, 1.0, 1.0, 1.0));

    let mut tiles = Vec::with_capacity(Tile::MAP_HEIGHT as usize * Tile::MAP_WIDTH as usize);

    for y in 0..Tile::MAP_HEIGHT {
        for x in 0..Tile::MAP_WIDTH {
            let tile_component = Tile::default();
            let mut transform = Transform::default();
            let (xpos, ypos) = Tile::get_pos(x, y);
            transform.set_translation_xyz(xpos, ypos, -1.0);
            transform.set_scale(Vector3::new(
                Tile::SIZE / Tile::SPRITE_SIZE,
                Tile::SIZE / Tile::SPRITE_SIZE,
                1.0,
            ));

            let entity = world
                .create_entity()
                .with(tile_component)
                .with(transform)
                .with(sprite_render.clone())
                .with(tint)
                .build();

            tiles.push(entity);
        }
    }
    let tiles = TileEntities(tiles);
    world.insert(tiles);
}

#[derive(Default)]
pub struct TileSystem;

impl<'s> System<'s> for TileSystem {
    type SystemData = (
        ReadStorage<'s, Tile>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Tint>,
        ReadExpect<'s, Time>,
    );

    fn run(&mut self, (tiles, transforms, mut tints, time): Self::SystemData) {
        use noise::{NoiseFn, Perlin};
        let noise = Perlin::new();

        (&tiles, &transforms, &mut tints)
            .par_join()
            .for_each(|(_tile, transform, tint)| {
                let (idx, idy) =
                    Tile::get_indices(transform.translation().x, transform.translation().y);
                let xf = idx as f64 / Tile::MAP_WIDTH as f64 * 1.5;
                let yf = idy as f64 / Tile::MAP_HEIGHT as f64 * 1.5;
                let tf = time.frame_number() as f64 / 1000.0;

                let noise_val = noise.get([xf, yf, tf]) as f32;

                let hue = noise_val * std::f32::consts::TAU;
                let saturation = f32::min(Hsl::from(tint.0).saturation + 0.001, 1.0);
                tint.0 = Hsl::new(RgbHue::from_radians(hue), saturation, 0.5).into();
            });
        println!(
            "average FPS: {}",
            time.frame_number() as f64 / time.absolute_time_seconds()
        );
    }
}
