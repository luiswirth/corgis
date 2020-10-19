use crate::{
    brain::{BodyPerception, Brain, Decision, EnvironmentPerception},
    corgi::{Corgi, Physique},
    genes::Genome,
    universe::{Universe, Values},
};
use amethyst::{
    assets::Handle,
    core::{
        math::{Vector2, Vector3},
        transform::Transform,
    },
    ecs::prelude::*,
    renderer::{palette::Hsl, resources::Tint, SpriteRender, SpriteSheet},
};
use rand::{thread_rng, Rng};
use rand_distr::{Distribution, Uniform};

const MIN_CORGI_COUNT: u32 = 10;

#[derive(Default)]
pub struct SpawnerSystem;

impl<'s> System<'s> for SpawnerSystem {
    type SystemData = (
        WriteStorage<'s, Corgi>,
        WriteStorage<'s, Brain>,
        WriteStorage<'s, BodyPerception>,
        WriteStorage<'s, EnvironmentPerception>,
        WriteStorage<'s, Decision>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Physique>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, Tint>,
        Entities<'s>,
        ReadExpect<'s, Handle<SpriteSheet>>,
        WriteExpect<'s, Values>,
    );

    fn run(
        &mut self,
        (
            mut corgis,
            mut brains,
            mut body_perceptions,
            mut environment_perceptions,
            mut decisions,
            mut transforms,
            mut physiques,
            mut sprite_renderers,
            mut tints,
            entities,
            sprite_sheet,
            mut values,
        ): Self::SystemData,
    ) {
        for (e, corgi) in (&*entities, &corgis).join() {
            if corgi.energy < 0.0 {
                values.corgi_count -= 1;
                entities.delete(e).unwrap();
            }
        }

        let x_pos_distr = Uniform::new(0.0, Universe::WIDTH_PIXEL);
        let y_pos_distr = Uniform::new(0.0, Universe::HEIGHT_PIXEL);

        let sprite_render = SpriteRender::new(sprite_sheet.clone(), 0);

        let mut rng = thread_rng();
        for _ in values.corgi_count..MIN_CORGI_COUNT {
            let corgi = Corgi {
                uuid: rng.gen(),
                name: String::from("SomeCorgi"),
                generation: 0,
                age: 0,

                energy: Corgi::INITIAL_ENERGY,

                genes: Genome::random(&mut rng),
            };

            let brain = Brain::new(corgi.genes.brain.clone());

            let body_perception = BodyPerception::default();
            let environment_perception = EnvironmentPerception::default();
            let decision = Decision::default();

            let mut local_transform = Transform::default();
            local_transform.set_scale(Vector3::new(2.0, 2.0, 2.0));
            local_transform.set_translation_xyz(
                x_pos_distr.sample(&mut rng),
                y_pos_distr.sample(&mut rng),
                0.0,
            );
            local_transform.set_scale(Vector3::new(3.0, 3.0, 3.0));

            let physique = Physique {
                mass: 1.0,

                velocity: Vector2::from_element(0.0),
                force: Vector2::from_element(0.0),
            };

            let tint = Tint(Hsl::new(0.0, 1.0, 0.5).into());

            entities
                .build_entity()
                .with(corgi, &mut corgis)
                .with(brain, &mut brains)
                .with(body_perception, &mut body_perceptions)
                .with(environment_perception, &mut environment_perceptions)
                .with(decision, &mut decisions)
                .with(local_transform, &mut transforms)
                .with(physique, &mut physiques)
                .with(sprite_render.clone(), &mut sprite_renderers)
                .with(tint, &mut tints)
                .build();
        }

        values.corgi_count = u32::max(values.corgi_count, MIN_CORGI_COUNT);
    }
}
