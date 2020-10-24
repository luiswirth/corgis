use crate::{
    brain::{PerceiveBodySystem, PerceiveEnvironmentSystem, ThinkSystem},
    corgi::{movement::MovementSystem, reproduce::ReproduceSystem, spawner::SpawnerSystem},
    universe::{energy::EnergySystem, tile::TileSystem},
};
use amethyst::{
    core::bundle::SystemBundle,
    ecs::prelude::{DispatcherBuilder, World},
    error::Error,
};

// Most systems depend on the brain system, because the order matters.
// Perception collection has to happen before the brain system and
// acting according to decisions has to happen after the brain system.

pub struct CorgiBundle;

impl<'a, 'b> SystemBundle<'a, 'b> for CorgiBundle {
    fn build(
        self,
        _world: &mut World,
        builder: &mut DispatcherBuilder<'a, 'b>,
    ) -> Result<(), Error> {
        builder.add(SpawnerSystem::default(), "spawn_system", &[]);
        builder.add(TileSystem::default(), "tile_system", &[]);
        builder.add(EnergySystem::default(), "energy_system", &[]);

        builder.add(PerceiveBodySystem::default(), "perceive_body_system", &[]);
        builder.add(
            PerceiveEnvironmentSystem::default(),
            "perceive_environment_system",
            &[],
        );

        builder.add(
            ThinkSystem::default(),
            "brain_system",
            &["perceive_body_system", "perceive_environment_system"],
        );

        builder.add(MovementSystem::default(), "move_system", &["brain_system"]);
        builder.add(
            ReproduceSystem::default(),
            "reproduction_system",
            &["brain_system"],
        );
        Ok(())
    }
}
