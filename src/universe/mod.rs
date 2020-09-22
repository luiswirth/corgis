pub mod energy;
pub mod tile;

use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    controls::FlyControlTag,
    core::{timing::Time, transform::Transform},
    ecs::prelude::*,
    prelude::*,
    renderer::{
        palette::{Hsv, RgbHue},
        Camera, ImageFormat, SpriteSheet, SpriteSheetFormat, Texture,
    },
};
use tile::Tile;

#[derive(Default)]
pub struct Universe {
    //sprite_sheet_handle: Option<Handle<SpriteSheet>>,
}

pub struct Values {
    pub corgi_count: u32,
    pub color: Hsv,
    pub epsilon: f32,
}

impl Universe {
    pub const WIDTH_TILE: u32 = 50;
    pub const HEIGHT_TILE: u32 = 30;

    pub const WIDTH_PIXEL: f32 = Universe::WIDTH_TILE as f32 * Tile::SIZE;
    pub const HEIGHT_PIXEL: f32 = Universe::HEIGHT_TILE as f32 * Tile::SIZE;
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
        tile::create_tiles(world);
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

const CAMERA_ZOOM: f32 = 50.0;

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(
        Universe::WIDTH_PIXEL * 0.5,
        Universe::HEIGHT_PIXEL * 0.5,
        1.0,
    );

    world
        .create_entity()
        .with(Camera::standard_2d(16.0 * CAMERA_ZOOM, 9.0 * CAMERA_ZOOM))
        .with(FlyControlTag)
        .with(transform)
        .build();
}
