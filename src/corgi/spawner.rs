use std::sync::Mutex;

use crate::{
    brain::Brain,
    corgi::{Corgi, Physique},
    genes::Genome,
    universe::{Universe, Values},
};
use amethyst::{
    assets::Handle,
    core::{
        math::{Vector2, Vector3},
        transform::Transform,
    },
    ecs::prelude::*,
    renderer::{palette::Hsl, resources::Tint, SpriteRender, SpriteSheet},
};
use rand::{thread_rng, Rng};
use rand_distr::{Distribution, Uniform};
use rayon::prelude::*;

const MIN_CORGI_COUNT: u32 = 10;

#[derive(Default)]
pub struct SpawnerSystem;

impl<'s> System<'s> for SpawnerSystem {
    type SystemData = (
        WriteStorage<'s, Corgi>,
        WriteStorage<'s, Brain>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Physique>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, Tint>,
        Entities<'s>,
        ReadExpect<'s, Handle<SpriteSheet>>,
        WriteExpect<'s, Values>,
    );

    fn run(
        &mut self,
        (
            corgis,
            brains,
            transforms,
            physiques,
            sprite_renderers,
            tints,
            entities,
            sprite_sheet,
            values,
        ): Self::SystemData,
    ) {
        for (e, corgi) in (&*entities, &corgis).join() {
            if corgi.energy < 0.0 {
                values.corgi_count -= 1;
                entities.delete(e).unwrap();
            }
        }

        let x_pos_distr = Uniform::new(0.0, Universe::WIDTH_PIXEL);
        let y_pos_distr = Uniform::new(0.0, Universe::HEIGHT_PIXEL);

        let corgis = Mutex::new(corgis);
        let brains = Mutex::new(brains);
        let transforms = Mutex::new(transforms);
        let physiques = Mutex::new(physiques);
        let sprite_renderers = Mutex::new(sprite_renderers);
        let tints = Mutex::new(tints);

        let sprite_render = SpriteRender::new(sprite_sheet.clone(), 1);

        (values.corgi_count..MIN_CORGI_COUNT)
            .into_par_iter()
            .for_each(|_| {
                let mut rng = thread_rng();

                let corgi = Corgi {
                    uuid: rng.gen(),
                    name: String::from("SomeCorgi"),
                    generation: 0,
                    age: 0,

                    energy: Corgi::INITIAL_ENERGY,

                    genes: Genome::random(&mut rng),
                };

                let mut local_transform = Transform::default();
                local_transform.set_scale(Vector3::new(2.0, 2.0, 2.0));
                local_transform.set_translation_xyz(
                    x_pos_distr.sample(&mut rng),
                    y_pos_distr.sample(&mut rng),
                    0.0,
                );

                let brain = Brain::new(corgi.genes.brain.clone());

                let physique = Physique {
                    mass: 1.0,

                    velocity: Vector2::from_element(0.0),
                    force: Vector2::from_element(0.0),
                };

                let tint = Tint(Hsl::new(0.0, 1.0, 0.5).into());

                entities
                    .build_entity()
                    .with(corgi, &mut corgis.lock().unwrap())
                    .with(brain, &mut brains.lock().unwrap())
                    .with(local_transform, &mut transforms.lock().unwrap())
                    .with(physique, &mut physiques.lock().unwrap())
                    .with(sprite_render.clone(), &mut sprite_renderers.lock().unwrap())
                    .with(tint, &mut tints.lock().unwrap())
                    .build();
            });

        values.corgi_count = u32::max(values.corgi_count, MIN_CORGI_COUNT);
    }
}
