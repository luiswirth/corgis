use crate::{
    corgi::{Corgi, CorgiBrain},
    genes::Genes,
    universe::Universe,
};

use crate::neural_network::NeuralNetwork;
use amethyst::{
    assets::Handle,
    core::transform::Transform,
    ecs::prelude::{Entities, Join, ReadExpect, System, WriteStorage},
    renderer::{SpriteRender, SpriteSheet},
};
use na::Vector2;
use rand::thread_rng;

pub struct SpawnerSystem {
    counter: u32,
}

impl SpawnerSystem {
    pub fn new() -> Self {
        Self { counter: 0 }
    }
}

impl<'s> System<'s> for SpawnerSystem {
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
        for (e, corgi) in (&*entities, &corgis).join() {
            if corgi.energy < 0.0 {
                self.counter -= 1;
                entities.delete(e).unwrap();
            }
        }

        let mut local_transform = Transform::default();
        local_transform.set_translation_xyz(Universe::WIDTH / 2.0, Universe::HEIGHT / 2.0, 0.0);

        let sprite_render = SpriteRender::new(sprite_sheet.clone(), 1);

        let mut rng = thread_rng();

        for _ in self.counter..50 {
            let genes = Genes::random(&mut rng);

            entities
                .build_entity()
                .with(local_transform.clone(), &mut transforms)
                .with(
                    Corgi {
                        name: String::from("SomeCorgi"),
                        color: [1.0, 0.0, 0.0, 1.0],
                        energy: Corgi::INITAL_ENERGY,

                        mass: 1.0,
                        velocity: Vector2::from_element(0.0),
                        force: Vector2::from_element(0.0),

                        genes: genes.clone(),

                        brain: CorgiBrain {
                            neural_network: NeuralNetwork::new(genes.brain.clone()),
                        },

                        will_to_reproduce: false,
                    },
                    &mut corgis,
                )
                .with(sprite_render.clone(), &mut sprite_renderers)
                .build();
        }
        self.counter = 50;
    }
}
