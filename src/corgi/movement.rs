use crate::{
    brain::Decision,
    consts::TIME_UNIT,
    corgi::{Corgi, Physique},
    universe::Universe,
};
use amethyst::{
    core::{math::Vector2, transform::Transform},
    derive::SystemDesc,
    ecs::prelude::*,
    renderer::resources::Tint,
};

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
        ReadStorage<'s, Decision>,
    );

    fn run(
        &mut self,
        (mut corgis, mut transforms, mut physiques, mut tints, decisions): Self::SystemData,
    ) {
        // TODO: parallelize again
        for (corgi, transform, physique, tint, decision) in (
            &mut corgis,
            &mut transforms,
            &mut physiques,
            &mut tints,
            &decisions,
        )
            .join()
        {
            physique.force += decision.force.0;
            physique.force += physique.velocity * -FRICTION;

            let acceleration = physique.force / physique.mass;
            physique.velocity += acceleration;

            let distance = physique.velocity * TIME_UNIT;

            let mut movement_work = distance.dot(&physique.force);
            movement_work *= Corgi::MOVEMENT_WORK;

            let life_work = 0.05;

            corgi.energy -= movement_work + life_work;

            transform.prepend_translation_x(distance[0]);
            transform.prepend_translation_y(distance[1]);

            let angle = f32::atan2(physique.velocity.y, physique.velocity.x);
            transform.set_rotation_2d(angle);

            physique.force = Vector2::new(0.0, 0.0);

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
            *tint = Tint(decision.color.0.into());
        }
    }
}
