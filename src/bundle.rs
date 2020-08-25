use crate::systems::{BrainSystem, MovementSystem, SpawnerSystem};
use amethyst::{
    core::bundle::SystemBundle,
    ecs::prelude::{DispatcherBuilder, World},
    error::Error,
};

/// A bundle is a convenient way to initialise related resources, components and systems in a
/// world. This bundle prepares the world for a game of pong.
pub struct CorgiBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for CorgiBundle {
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        builder.add(SpawnerSystem::new(), "corgi_spawn_system", &[]);
        builder.add(BrainSystem, "corgi_brain_system", &[]);
        builder.add(MovementSystem, "corgi_move_system", &["corgi_brain_system"]);
        Ok(())
    }
}
