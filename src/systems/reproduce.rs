use crate::{brain::Brain, corgi::Corgi};

use crate::{neural_network::NeuralNetwork, util::types::Color};
use amethyst::{
    assets::Handle,
    core::transform::Transform,
    ecs::prelude::{Entities, Join, ReadExpect, System, WriteStorage},
    renderer::{SpriteRender, SpriteSheet},
};
use na::Vector2;
use rand::{thread_rng, Rng};

pub struct ReproduceSystem;

impl ReproduceSystem {}

const REPRODUCTION_ENERGY: f32 = 100.0;

impl<'s> System<'s> for ReproduceSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Corgi>,
        WriteStorage<'s, SpriteRender>,
        Entities<'s>,
        ReadExpect<'s, Handle<SpriteSheet>>,
    );

    fn run(
        &mut self,
        (mut transforms, mut corgis, mut sprite_renderers, entities, sprite_sheet): Self::SystemData,
    ) {
        let mut rng = thread_rng();
        let sprite_render = SpriteRender::new(sprite_sheet.clone(), 1);

        let mut new_corgis: Vec<(Corgi, Transform)> = Vec::new();

        for (mut corgi, transform) in (&mut corgis, &transforms).join() {
            if corgi.reproduction_will && corgi.energy >= REPRODUCTION_ENERGY {
                let mut genes = corgi.genes.clone();
                genes.mutate(&mut rng);

                new_corgis.push((
                    Corgi {
                        uuid: rng.gen(),
                        name: String::from("SomeCorgi"),
                        generation: 0,

                        energy: 50.0,
                        mass: 1.0,
                        velocity: Vector2::from_element(0.0),
                        force: Vector2::from_element(0.0),

                        genes: genes.clone(),

                        brain: Brain::new(genes.brain.clone()),

                        color: Color::new(0.0, 0.0, 0.0),
                        reproduction_will: false,
                    },
                    transform.clone(),
                ));

                corgi.energy -= REPRODUCTION_ENERGY;
            }
        }

        for (c, t) in new_corgis {
            entities
                .build_entity()
                .with(c, &mut corgis)
                .with(t, &mut transforms)
                .with(sprite_render.clone(), &mut sprite_renderers)
                .build();
        }
    }
}
