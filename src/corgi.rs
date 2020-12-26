use crate::{
    intelligence::IntelligenceBundle,
    loader::MyAssets,
    universe::{UNIVERSE_HEIGHT, UNIVERSE_WIDTH},
};
use bevy::prelude::*;
use bevy_rapier2d::rapier::{dynamics::RigidBodyBuilder, geometry::ColliderBuilder};
use rand::{distributions::Uniform, prelude::Distribution};

const MIN_CORGI_COUNT: usize = 1;
const CORGI_ENERGY_SPAWNED: f32 = 100.0;
const CORGI_ENERGY_BORN: f32 = 100.0;

pub struct Corgi;
pub struct Energy(pub f32);
pub struct Age(pub usize);
pub struct Generation(pub usize);
//struct Gene;
//struct Brain;

#[derive(Bundle)]
pub struct CorgiBundle {
    pub _tag: Corgi,
    pub energy: Energy,
    pub age: Age,
    pub generation: Generation,
    pub rigid_body: RigidBodyBuilder,
    pub collider: ColliderBuilder,
    pub pbr: PbrBundle,
    pub intelligence: IntelligenceBundle,
}

impl CorgiBundle {
    fn new_spawned(pos: Vec2, mesh: Handle<Mesh>, material: Handle<StandardMaterial>) -> Self {
        let pbr = PbrBundle {
            mesh,
            material,
            ..Default::default()
        };

        Self {
            _tag: Corgi,
            energy: Energy(CORGI_ENERGY_SPAWNED),
            age: Age(0),
            generation: Generation(0),
            rigid_body: RigidBodyBuilder::new_dynamic().translation(pos.x, pos.y),
            collider: ColliderBuilder::cuboid(10.0, 10.0).density(1.0),
            pbr,
            intelligence: IntelligenceBundle::default(),
        }
    }
}

pub fn corgi_spawner(commands: &mut Commands, query: Query<&Corgi>, assets: Res<MyAssets>) {
    let mut rng = rand::thread_rng();
    let x_pos_distr = Uniform::new(0.0, UNIVERSE_WIDTH);
    let y_pos_distr = Uniform::new(0.0, UNIVERSE_HEIGHT);
    for _ in query.iter().len()..MIN_CORGI_COUNT {
        let x = x_pos_distr.sample(&mut rng);
        let y = y_pos_distr.sample(&mut rng);
        commands.spawn(CorgiBundle::new_spawned(
            Vec2::new(x, y),
            assets.corgi_mesh.clone(),
            assets.corgi_material.clone(),
        ));
    }
}
