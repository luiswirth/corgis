use crate::systems::{CogiMoveSystem, CogiSpawnSystem, CogiBrainSystem};
use amethyst::{
    core::bundle::SystemBundle,
    ecs::prelude::{DispatcherBuilder, World},
    error::Error,
};

/// A bundle is a convenient way to initialise related resources, components and systems in a
/// world. This bundle prepares the world for a game of pong.
pub struct PongBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for PongBundle {
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        builder.add(CogiMoveSystem, "cogi_move_system", &[]);
        builder.add(CogiSpawnSystem::new(), "cogi_spawn_system", &[]);
        builder.add(CogiBrainSystem, "cogi_brain_system", &[]);
        Ok(())
    }
}
