use std::sync::Mutex;

use crate::{
    brain::Brain,
    corgi::{Corgi, Physique},
    universe::Values,
};
use amethyst::{
    assets::Handle,
    core::{math::Vector2, transform::Transform},
    ecs::prelude::*,
    renderer::{palette::Hsl, resources::Tint, SpriteRender, SpriteSheet},
};
use rand::{thread_rng, Rng};

#[derive(Default)]
pub struct ReproduceSystem;

const MATURITY_AGE: u32 = 1_000;

impl<'s> System<'s> for ReproduceSystem {
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
            mut corgis,
            mut brains,
            mut transforms,
            mut physiques,
            mut sprite_renderers,
            mut tints,
            entities,
            sprite_sheet,
            values,
        ): Self::SystemData,
    ) {
        let sprite_render = SpriteRender::new(sprite_sheet.clone(), 1);

        let new_corgis: Mutex<Vec<(Corgi, Brain, Transform)>> = Mutex::default();

        (&mut corgis, &brains, &transforms)
            .par_join()
            .for_each(|(mut corgi, brain, transform)| {
                if corgi.age >= MATURITY_AGE
                    && decisions.reproduction_will
                    && corgi.energy >= Corgi::REPRODUCTION_WORK
                {
                    let mut rng = thread_rng();

                    corgi.energy -= Corgi::REPRODUCTION_WORK;

                    let mut genes = corgi.genes.clone();
                    genes.mutate(&mut rng);

                    let corgi = Corgi {
                        uuid: rng.gen(),
                        name: String::from("SomeCorgi"),
                        generation: corgi.generation + 1,
                        age: 0,

                        energy: Corgi::BORN_ENERGY,
                        genes: genes.clone(),
                    };

                    let brain = Brain::new(genes.brain);

                    let mut new_corgis = new_corgis.lock().unwrap();
                    new_corgis.push((corgi, brain, transform.clone()));
                }
            });

        let new_corgis = new_corgis.into_inner().unwrap();

        let physique = Physique {
            mass: 1.0,

            velocity: Vector2::from_element(0.0),
            force: Vector2::from_element(0.0),
        };

        for (corgi, brain, transform) in new_corgis {
            entities
                .build_entity()
                .with(corgi, &mut corgis)
                .with(brain, &mut brains)
                .with(transform, &mut transforms)
                .with(physique.clone(), &mut physiques)
                .with(sprite_render.clone(), &mut sprite_renderers)
                .with(Tint(Hsl::new(1.0, 1.0, 0.5).into()), &mut tints)
                .build();

            values.corgi_count += 1;
        }

        println!("corgi count: {}", values.corgi_count);
    }
}
