pub mod brain;
pub mod decision;
pub mod perception;

// for testing the IO
mod test;
use super::test;

use bevy::prelude::*;
use bevy_rapier2d::{physics::RigidBodyHandleComponent, rapier::dynamics::RigidBodySet};

use crate::corgi::{Corgi, Energy};

/// The Brain system needs to have some sort of IO.
/// Each corgi has his own neural network, which is run once per frame.
/// The inputs into the brain are called it's *Perceptions*.
/// The outputs are called it's *Decisions*.
///
/// There can be multiple ways of perceiving the environment.
/// Each has it's own system which only writes the perceived
/// values into a dedicated component.
///
/// The `think` system accesses all perception and decision components.
/// It then merges all perception components into one `Vector` and
/// runs the neural network once for every corgi.
/// The decisions `Vector` is then split up into decision components.
///
/// Every decision component also has his own dedicated systems, which applies
/// the decision values to the behaviour of the corgi.
///
/// This is all done in 3 different stages and one additional one for the cycle transition.
/// The stages are run sequentially and parallel internaly.
/// Perceive-Stage => Think-Stage => Decide-Stage => Transition-Stage
///
/// The perception and decisions need to be split up into components
/// in order for the system to run in parallel, because then the RW access doesn't conflict.
/// We should also hold on to the allocated memory.
///
/// There persits one problem.
/// Accumulating the perceptions is easy
/// and can be done in an arbitrary but deterministic order.
/// The order has to stay always the same for this neural network.
///
/// Splitting up the decisions is a much harder problem,
/// since we need to know which values go into which components.
///
/// To find out the perception composition and the decisions disassembly,
/// an inital dry-run is done on brain creation.
/// For these we use "mock objects" to imitate the components
/// and count every perception insertion and decision retrieval.
///
/// This structure determiniton can also be used for some checks.
/// Any invalid input or output shape can be reported, if it doesn't match the brain shape.
///
/// For the dry-run we need the mock objects. After it we need the actual stores.
/// How do we switch? Do we replace the components (change archetype) or
/// does the component change it's state (for instance enum).

/// Perceive => Think => Decide => Transition
pub struct IntelligencePlugin;

impl Plugin for IntelligencePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            // perception stage (multiple systems) -- fill InputStore values
            .add_stage("perceive", SystemStage::parallel())
            // think / compute stage (one neural network system) -- run nn, take all InputStore values (shape check), provide empty OutputStore
            .add_stage_after("perceive", "think", SystemStage::parallel())
            // decide stage (multiple systems) -- use all OutputStore values
            .add_stage_after("think", "decide", SystemStage::parallel())
            // transition stage (one system) -- remove OutputStore Comps (empty check), add InputStore Comps (empty)
            .add_stage_after("decide", "transition", SystemStage::parallel())
            
            // --- default systems ---
            // dry-run startup system (determine neural network shape)
            .add_system(dry_run.system());
            .add_system_to_stage("think", think.system())
            .add_system_to_stage("transition", transition.system());

            // debug systems
            .add_system_to_stage("perceive", test::perceive.system())
            .add_system_to_stage("decide", test::decide.system())
    }

    fn name(&self) -> &str {
        "IntelligencePlugin"
    }
}

pub struct Brain;

impl Default for Brain {
    fn default() -> Self {
        Self::new_random()
    }
}

pub struct IntelligenceBundle {
    brain: Brain,
    perception: PerceptionBundle,
    decision: DecisionBundle,
    perception_dry_run: ThoughtDryRun,
    decision_dry_run: ThoughtDryRun,
}
impl Default for IntelligenceBundle {
    fn default() -> Self {
        Self {
            state: IntelligenceState::DryRun,
            brain: Brain::default(),
            perception: PerceptionBundle::default(),
            decision: DecisionBundle::default(),
            perception_dry_run: ThoughtDryRun::default(),
            decision_dry_run: ThoughtDryRun::default(),
        }
    }
}

trait BrainStore {
    fn len(&self) -> usize;
}

trait BrainInputStore: BrainStore {
    fn put(&mut self, value: f32);
    fn extend<I>(&mut self, values: I)
    where
        I: Iterator<Item = f32>;
}

trait BrainOutputStore: BrainStore {
    fn take(&mut self) -> f32;
    fn take_multiple(&mut self, n: usize) -> Box<[f32]>;
}

// PerceptionComponent
#[derive(Default, Clone, Debug)]
pub struct Perception {
    vec: Vec<f32>,
}
// DecisionComponent
#[derive(Default, Clone, Debug)]
pub struct Decision {
    vec: Vec<f32>,
}

// TODO: write procedural attribute macro
perception!(BodyPerception, VisionPerception);
#[derive(Bundle, Default)]
pub struct PerceptionBundle {
    body: BodyPerception,
    vision: VisionPerception,
}

decision!(MovementDecision, ReproductionDecision);
#[derive(Bundle, Default)]
pub struct DecisionBundle {
    movement: MovementDecision,
    reproduction: ReproductionDecision,
}

impl BrainStore for Perception {
    fn len(&self) -> usize {
        self.vec.len()
    }
}

// always have to be filled in the same order
impl BrainInputStore for Perception {
    fn put(&mut self, value: f32) {
        self.vec.push(value);
    }

    fn extend<I>(&mut self, values: I)
    where
        I: Iterator<Item = f32>,
    {
        for value in values {
            self.put(value.to_owned());
        }
    }
}

impl BrainStore for Decision {
    fn len(&self) -> usize {
        self.vec.len()
    }
}

// always has to be emptied in the same order
impl BrainOutputStore for Decision {
    fn take(&mut self) -> f32 {
        self.vec.pop().expect("No more outputs left")
    }

    fn take_multiple(&mut self, n: usize) -> Vec<f32> {
        let mut vec = Vec::with_capacity(n);
        for _ in 0..n {
            vec.push(self.take());
        }
        vec
    }
}

// initally corgi needs BodyPerception and VisionPerception

fn think(query: Query<(&Brain, &PerceptionBundle, &DecisionBundle)>) {
    for (brain, perception, decision) in query.iter() {
        // collect all BrainInputStores together -> always same ordering of values
        let perception = perception
            .body
            .0
            .vec
            .into_iter()
            .zip(perception.vision.0.vec.into_iter());
    }
}

fn transition(commands: &mut Commands, query: Query<(Entity, &DecisionBundle)>) {
    for (entity, decision) in query.iter() {

        // check if all outputs have been consumed
        assert_eq!(decision.movement.0.len(), 0);
        assert_eq!(decision.reproduction.0.len(), 0);

        // empty Perceptions and Decisions
    }
}

// DRY RUN

#[derive(Default, Clone, Debug)]
struct ThoughtDryRun {
    count: usize,
}

impl BrainStore for ThoughtDryRun {
    fn len(&self) -> usize {
        self.count
    }
}

impl BrainInputStore for ThoughtDryRun {
    fn put(&mut self, _: f32) {
        self.count += 1;
    }

    fn extend<I>(&mut self, values: I)
    where
        I: Iterator<Item = f32>,
    {
        self.count += values.count();
    }
}

impl BrainOutputStore for ThoughtDryRun {
    fn take(&mut self) -> f32 {
        self.count += 1;
        0.0
    }

    fn take_multiple(&mut self, n: usize) -> Vec<f32> {
        self.count += n;
        vec![]
    }
}

fn dry_run(query: Query<(PerceptionDryRun, DecisionDryRun), Added<IntelligenceBundle>>) {
    // should the dryrun types persist after the dry run or should they be removed?
    todo!()
}

macro_rules! perception {
    ($($i:ident), *) => {
        $(
            #[derive(Default, Clone, Debug)]
            pub struct $i(pub Perception);
        )*
    };
}

macro_rules! decision {
    ($($i:ident), *) => {
        $(
            #[derive(Default, Clone, Debug)]
            pub struct $i(pub Decision);
        )*
    }
}

