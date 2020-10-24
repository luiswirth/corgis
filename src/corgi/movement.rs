use crate::{
    brain::Decision,
    consts::TIME_UNIT,
    corgi::{Corgi, Physique},
    universe::Universe,
};
use amethyst::{
    core::{
        math::{Vector2, Vector3},
        transform::Transform,
    },
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
            //let angle = f32::atan2(physique.velocity.y, physique.velocity.x);
            let orientation_change = (decision.orientation_change.0 * 2.0 - 0.5) / 80.0;
            transform.rotate_2d(orientation_change);

            let scale = corgi.energy / Corgi::INITIAL_ENERGY + 1.0;
            transform.set_scale(Vector3::new(scale, scale, scale));

            let force = decision.force.0 * 10.0;

            let orientation = transform.rotation().euler_angles().2;
            physique.force.x += force * f32::cos(orientation);
            physique.force.y += force * f32::sin(orientation);

            physique.force += physique.velocity * -FRICTION;

            let acceleration = physique.force / physique.mass;
            physique.velocity += acceleration;

            let distance = physique.velocity * TIME_UNIT;

            let mut movement_work = distance.dot(&physique.force);
            movement_work *= Corgi::MOVEMENT_WORK;

            let age_work = (corgi.age as f32 - 3000.0).exp();
            let life_work = 0.05 + age_work;

            corgi.energy -= movement_work + life_work;

            transform.prepend_translation_x(distance[0]);
            transform.prepend_translation_y(distance[1]);

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
