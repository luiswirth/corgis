use crate::{
    brain::*,
    corgi::{Corgi, Physique},
    universe::tile::TileEntities,
};
use amethyst::{
    core::Transform,
    ecs::{prelude::*, System},
    renderer::{palette::Hsl, resources::Tint},
};

#[derive(Component, Default, Clone)]
pub struct EnvironmentPerception(pub BrainInput);

#[derive(Default)]
pub struct PerceiveEnvironmentSystem;

impl<'s> System<'s> for PerceiveEnvironmentSystem {
    type SystemData = (
        WriteStorage<'s, EnvironmentPerception>,
        ReadStorage<'s, Corgi>,
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Physique>,
        ReadStorage<'s, Tint>,
        ReadExpect<'s, TileEntities>,
    );

    fn run(
        &mut self,
        (mut perceptions, corgis, transforms, physiques, tints, tile_entities): Self::SystemData,
    ) {
        for (_corgi, transform, physique, perception) in
            (&corgis, &transforms, &physiques, &mut perceptions).join()
        {
            if let Some(tile_entity) =
                tile_entities.get_at_pos(transform.translation().x, transform.translation().y)
            {
                if let Some(tile_tint) = tints.get(*tile_entity) {
                    perception.0.data.push(0.0);
                    perception.0.perceive(tile_tint.0.color.r);
                    perception.0.perceive(tile_tint.0.color.g);
                    perception.0.perceive(tile_tint.0.color.b);
                    perception.0.perceive(physique.velocity.clone_owned());
                }
            }
        }
    }
}
