use crate::cogi::Cogi;
use crate::{ARENA_HEIGHT, ARENA_WIDTH};
use amethyst::{
    core::{timing::Time, transform::Transform},
    derive::SystemDesc,
    ecs::prelude::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
};

/// This system is responsible for moving all balls according to their speed
/// and the time passed.
#[derive(SystemDesc)]
pub struct CogiMoveSystem;

impl<'s> System<'s> for CogiMoveSystem {
    type SystemData = (
        WriteStorage<'s, Cogi>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut cogis, mut locals, time): Self::SystemData) {
        // Move every ball according to its speed, and the time passed.
        for (cogi, transform) in (&mut cogis, &mut locals).join() {
            cogi.velocity[0] += cogi.force[0];
            cogi.force[0] = 0.0;
            cogi.velocity[1] += cogi.force[1];
            cogi.force[1] = 0.0;

            transform.prepend_translation_x(cogi.velocity[0] * time.delta_seconds());
            transform.prepend_translation_y(cogi.velocity[1] * time.delta_seconds());
            if transform.translation().x < 0.0 {
                transform.translation_mut().x = 0.0;
                cogi.velocity[0] *= -1.0;
            }
            if transform.translation().y < 0.0 {
                transform.translation_mut().y = 0.0;
                cogi.velocity[1] *= -1.0;
            }
            if transform.translation().x >= ARENA_WIDTH {
                transform.translation_mut().x = ARENA_WIDTH;
                cogi.velocity[0] *= -1.0;
            }
            if transform.translation().y >= ARENA_HEIGHT {
                transform.translation_mut().y = ARENA_HEIGHT;
                cogi.velocity[1] *= -1.0;
            }
        }
    }
}
