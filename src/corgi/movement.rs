use crate::{
    brain::Brain,
    consts::TIME_UNIT,
    corgi::{Corgi, Physique},
    universe::Universe,
};
use amethyst::{
    core::{timing::Time, transform::Transform},
    derive::SystemDesc,
    ecs::prelude::*,
    renderer::resources::Tint,
};
use crate::brain::Decisions;

#[derive(Default, SystemDesc)]
pub struct MovementSystem;

// expressed as multiple of delta frame count

const FRICTION: f32 = 0.001;

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        WriteStorage<'s, Corgi>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Physique>,
        WriteStorage<'s, Tint>,
        ReadStorage<'s, Decisions>,
    );

    fn run(
        &mut self,
        (mut corgis, mut transforms, mut physiques, mut tints, decisions): Self::SystemData,
    ) {
        (
            &mut corgis,
            &mut transforms,
            &mut physiques,
            &mut tints,
            &decisions,
        )
            .par_join()
            .for_each(|(corgi, transform, physique, tint, decision)| {
                corgi.force += decisions.force.0;
                corgi.force += physique.velocity * -FRICTION;

                let acceleration = physique.force / physique.mass;
                physique.velocity += acceleration;

                let distance = corgi.velocity * TIME_UNIT;

                let mut movement_work = distance.dot(&corgi.force);
                movement_work *= Corgi::MOVEMENT_WORK;

                let life_work = 0.05;

                corgi.energy -= movement_work + life_work;

                transform.prepend_translation_x(distance[0]);
                transform.prepend_translation_y(distance[1]);

                corgi.force = 0.0;

                if transform.translation().x < 0.0 {
                    transform.translation_mut().x = Universe::WIDTH_PIXEL;
                }
                if transform.translation().y < 0.0 {
                    transform.translation_mut().y = Universe::HEIGHT_PIXEL;
                }
                if transform.translation().x > Universe::WIDTH_PIXEL {
                    transform.translation_mut().x = 0.0;
                }
                if transform.translation().y > Universe::HEIGHT_PIXEL {
                    transform.translation_mut().y = 0.0;
                }

                corgi.age += 1;
                *tint = Tint(corgi.color.into());
            });
    }
}
