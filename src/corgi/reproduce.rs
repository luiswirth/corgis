use crate::na::Vector2;
use crate::{brain::Brain, corgi::Corgi, universe::Values};
use amethyst::{
    assets::Handle,
    core::transform::Transform,
    ecs::prelude::{Entities, Join, ReadExpect, System, WriteExpect, WriteStorage},
    renderer::{palette::Hsv, resources::Tint, SpriteRender, SpriteSheet},
};
use rand::{thread_rng, Rng};

pub struct ReproduceSystem;

impl ReproduceSystem {}

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
        let mut rng = thread_rng();
        let sprite_render = SpriteRender::new(sprite_sheet.clone(), 1);

        let mut new_corgis: Vec<(Corgi, Transform)> = Vec::new();

        for (mut corgi, transform) in (&mut corgis, &transforms).join() {
            if corgi.reproduction_will && corgi.energy >= Corgi::REPRODUCTION_WORK {
                let mut genes = corgi.genes.clone();
                genes.mutate(&mut rng);

                new_corgis.push((
                    Corgi {
                        uuid: rng.gen(),
                        name: String::from("SomeCorgi"),
                        generation: corgi.generation + 1,

                        energy: Corgi::BORN_ENERGY,
                        mass: 1.0,
                        velocity: Vector2::from_element(0.0),
                        force: Vector2::from_element(0.0),

                        genes: genes.clone(),

                        brain: Brain::new(genes.brain.clone()),

                        color: Hsv::new(0.0, 0.0, 0.0),
                        reproduction_will: false,
                    },
                    transform.clone(),
                ));

                corgi.energy -= Corgi::REPRODUCTION_WORK;
            }
        }

        for (c, t) in new_corgis {
            let generation = c.generation;

            let mut ent = entities
                .build_entity()
                .with(c, &mut corgis)
                .with(t, &mut transforms)
                .with(Tint(Hsv::new(1.0, 1.0, 1.0).into()), &mut tints);

            if generation >= 3 {
                ent = ent.with(sprite_render.clone(), &mut sprite_renderers);
            }
            if values.corgi_count < 30_000 {
                ent.build();
                values.corgi_count += 1;
            }
        }

        println!("{}", values.corgi_count);
    }
}
