use crate::universe::Cogi;
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
        ReadStorage<'s, Cogi>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (cogis, mut locals, time): Self::SystemData) {
        // Move every ball according to its speed, and the time passed.
        for (cogi, local) in (&cogis, &mut locals).join() {
            local.prepend_translation_x(cogi.velocity[0] * time.delta_seconds());
            local.prepend_translation_y(cogi.velocity[1] * time.delta_seconds());
        }
    }
}
