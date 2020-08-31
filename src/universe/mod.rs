pub mod tile;

use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::{timing::Time, transform::Transform},
    ecs::prelude::*,
    prelude::*,
    renderer::{
        palette::{Hsv, RgbHue},
        Camera, ImageFormat, SpriteSheet, SpriteSheetFormat, Texture,
    },
};
use na::Point2;
use tile::{Tile, TileType};

#[derive(Default)]
pub struct Universe {
    //sprite_sheet_handle: Option<Handle<SpriteSheet>>,
}

pub struct Values {
    pub corgi_count: u32,
    pub color: Hsv,
    pub epsilon: f32,
}

pub struct Tiles(pub Vec<Entity>);

impl Universe {
    pub const WIDTH: f32 = 3840.0 / 10.0;
    pub const HEIGHT: f32 = 2160.0 / 10.0;
}

impl SimpleState for Universe {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let StateData { world, .. } = data;

        initialise_camera(world);
        let sprite_sheet = load_sprite_sheet(world);
        world.insert(sprite_sheet);
        world.insert(Values {
            corgi_count: 0,
            color: Hsv::new(0.0, 1.0, 1.0),
            epsilon: 0.5,
        });
        //create_tiles(world);
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let StateData { world, .. } = data;
        let _time = world.fetch::<Time>();

        let Values { ref mut color, .. } = *world.fetch_mut::<Values>();
        color.hue = color.hue + 0.5;
        if color.hue.to_degrees() > 180.0 {
            color.hue = RgbHue::from_degrees(-180.0)
        }

        Trans::None
    }
}

#[allow(unused)]
fn create_tiles(world: &mut World) {
    let mut entities = Vec::new();
    for y in 0..20 {
        for x in 0..20 {
            let tile = Tile {
                position: Point2::new(x as f32 * Tile::SIZE, y as f32 * Tile::SIZE),
                ttype: TileType::Energy(0.0),
            };
            let entity = world.create_entity().with(tile).build();
            entities.push(entity);
        }
    }
    world.insert(Tiles(entities));

    //let map = TileMap::<Tile>::new(
    //Vector3::new(10, 10, 1),
    //Vector3::new(Tile::SIZE as u32, Tile::SIZE as u32, 1),
    //Some((*world.read_resource::<Handle<SpriteSheet>>()).clone()),
    //);
}

fn load_sprite_sheet(world: &mut World) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "texture/pong_spritesheet.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "texture/pong_spritesheet.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(Universe::WIDTH * 0.5, Universe::HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(Universe::WIDTH, Universe::HEIGHT))
        .with(transform)
        .build();
}
