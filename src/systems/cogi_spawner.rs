use crate::cogi::{self, Cogi, CogiBrain};
use crate::universe::{ARENA_HEIGHT, ARENA_WIDTH};

use crate::neural_network::NeuralNetwork;
use amethyst::{
    assets::Handle,
    core::transform::Transform,
    derive::SystemDesc,
    ecs::prelude::{Entities, Join, ReadExpect, System, WriteStorage},
    renderer::{SpriteRender, SpriteSheet},
};
use rand::{rngs::ThreadRng, thread_rng, Rng};

pub struct CogiSpawnSystem {
    counter: u32,
}

impl CogiSpawnSystem {
    pub fn new() -> Self {
        Self { counter: 0 }
    }
}

impl<'s> System<'s> for CogiSpawnSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Cogi>,
        WriteStorage<'s, SpriteRender>,
        Entities<'s>,
        ReadExpect<'s, Handle<SpriteSheet>>,
    );

    fn run(
        &mut self,
        (mut transforms, mut cogis, mut sprite_renderers, entities, sprite_sheet): Self::SystemData,
    ) {
        for (e, cogi) in (&*entities, &cogis).join() {
            if cogi.energy < 0.0 {
                self.counter -= 1;
                entities.delete(e).unwrap();
            }
        }

        let mut local_transform = Transform::default();
        local_transform.set_translation_xyz(ARENA_WIDTH / 2.0, ARENA_HEIGHT / 2.0, 0.0);

        let sprite_render = SpriteRender::new(sprite_sheet.clone(), 1);

        let mut rng = thread_rng();

        for _ in self.counter..50 {
            entities
                .build_entity()
                .with(local_transform.clone(), &mut transforms)
                .with(
                    Cogi {
                        name: String::from("SomeCogi"),
                        color: [1.0, 0.0, 0.0, 1.0],
                        energy: cogi::INITAL_ENERGY,

                        velocity: [rng.gen(), rng.gen()],
                        force: [0.0, 0.0],

                        brain: CogiBrain {
                            neural_network: NeuralNetwork::new_random(vec![5, 5, 7, 5, 2]),
                        },
                    },
                    &mut cogis,
                )
                .with(sprite_render.clone(), &mut sprite_renderers)
                .build();
        }
        self.counter = 50;
    }
}
