use crate::{brain::Brain, corgi::Corgi, genes::Genes, universe::Universe};

use crate::universe::Values;
use amethyst::{
    assets::Handle,
    core::transform::Transform,
    ecs::prelude::{Entities, Join, ReadExpect, System, WriteExpect, WriteStorage},
    renderer::{palette::Hsv, resources::Tint, SpriteRender, SpriteSheet},
};
use na::Vector2;
use rand::{thread_rng, Rng};

const MIN_COGI_COUNT: u32 = 10000;

pub struct SpawnerSystem;

impl<'s> System<'s> for SpawnerSystem {
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
            _sprite_renderers,
            mut tints,
            entities,
            sprite_sheet,
            mut values,
        ): Self::SystemData,
    ) {
        for (e, corgi) in (&*entities, &corgis).join() {
            if corgi.energy < 0.0 {
                values.corgi_count -= 1;
                entities.delete(e).unwrap();
            }
        }

        let mut local_transform = Transform::default();
        local_transform.set_translation_xyz(Universe::WIDTH / 2.0, Universe::HEIGHT / 2.0, 0.0);

        let _sprite_render = SpriteRender::new(sprite_sheet.clone(), 1);

        let mut rng = thread_rng();

        for _ in values.corgi_count..MIN_COGI_COUNT {
            let genes = Genes::random(&mut rng);

            entities
                .build_entity()
                .with(local_transform.clone(), &mut transforms)
                .with(
                    Corgi {
                        uuid: rng.gen(),
                        name: String::from("SomeCorgi"),
                        generation: 0,

                        energy: Corgi::INITAL_ENERGY,
                        mass: 1.0,
                        velocity: Vector2::from_element(0.0),
                        force: Vector2::from_element(0.0),

                        genes: genes.clone(),

                        brain: Brain::new(genes.brain.clone()),

                        color: Hsv::new(0.0, 0.0, 0.0),
                        reproduction_will: false,
                    },
                    &mut corgis,
                )
                //.with(sprite_render.clone(), &mut sprite_renderers)
                .with(Tint(Hsv::new(1.0, 1.0, 1.0).into()), &mut tints)
                .build();
        }
        values.corgi_count = u32::max(values.corgi_count, MIN_COGI_COUNT);
    }
}
