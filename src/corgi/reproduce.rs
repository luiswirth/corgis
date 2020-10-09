use std::sync::Mutex;

use crate::{brain::Brain, corgi::Corgi, na::Vector2, universe::Values};
use amethyst::{
    assets::Handle,
    core::transform::Transform,
    ecs::prelude::*,
    renderer::{palette::Hsl, resources::Tint, SpriteRender, SpriteSheet},
};
use rand::{thread_rng, Rng};

pub struct ReproduceSystem;

impl ReproduceSystem {}

const MATURITY_AGE: u32 = 1_000;

impl<'s> System<'s> for ReproduceSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Corgi>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, Tint>,
        Entities<'s>,
        ReadExpect<'s, Handle<SpriteSheet>>,
        WriteExpect<'s, Values>,
    );

    fn run(
        &mut self,
        (
            mut transforms,
            mut corgis,
            mut sprite_renderers,
            mut tints,
            entities,
            sprite_sheet,
            mut values,
        ): Self::SystemData,
    ) {
        let sprite_render = SpriteRender::new(sprite_sheet.clone(), 1);

        let new_corgis: Mutex<Vec<(Corgi, Transform)>> = Mutex::default();

        (&mut corgis, &transforms)
            .par_join()
            .for_each(|(mut corgi, transform)| {
                if corgi.age >= MATURITY_AGE
                    && corgi.reproduction_will
                    && corgi.energy >= Corgi::REPRODUCTION_WORK
                {
                    corgi.energy -= Corgi::REPRODUCTION_WORK;

                    let mut rng = thread_rng();
                    let mut genes = corgi.genes.clone();
                    genes.mutate(&mut rng);

                    let corgi = Corgi {
                        uuid: rng.gen(),
                        name: String::from("SomeCorgi"),
                        generation: corgi.generation + 1,
                        age: 0,

                        energy: Corgi::BORN_ENERGY,
                        mass: 1.0,
                        velocity: Vector2::from_element(0.0),
                        force: Vector2::from_element(0.0),

                        genes: genes.clone(),

                        brain: Brain::new(genes.brain.clone()),

                        color: Hsl::new(0.0, 0.0, 0.0),
                        reproduction_will: false,
                    };

                    let mut new_corgis = new_corgis.lock().unwrap();
                    new_corgis.push((corgi, transform.clone()));
                }
            });

        let new_corgis = new_corgis.into_inner().unwrap();

        for (corgi, transform) in new_corgis {
            entities
                .build_entity()
                .with(corgi, &mut corgis)
                .with(sprite_render.clone(), &mut sprite_renderers)
                .with(transform, &mut transforms)
                .with(Tint(Hsl::new(1.0, 1.0, 0.5).into()), &mut tints)
                .build();

            values.corgi_count += 1;
        }

        println!("{}", values.corgi_count);
    }
}
