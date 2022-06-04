use ordered_float::OrderedFloat;
use rand::prelude::*;

use lib_genetic_algorithm::{
    GaussianMutation, GeneticAlgorithm, RouletteWheelSelection, UniformCrossover,
};

use snake::Snake;

mod brain;
mod snake;
mod snake_individual;

const POPULATION_SIZE: usize = 2000;
const MUTATION_RATE: f32 = 0.15;
const MUTATION_STRENGTH: f32 = 0.3;
const GENERATION_LENGTH: usize = 10_000;

pub(crate) struct Evolution {
    population: Vec<Snake>,
    ga: GeneticAlgorithm<RouletteWheelSelection>,
    age: usize,
}

impl Evolution {
    pub(crate) fn random(rng: &mut dyn RngCore, game_grid_size: isize) -> Evolution {
        let population: Vec<Snake> = (0..POPULATION_SIZE)
            .map(|_| Snake::random(rng, game_grid_size))
            .collect();
        let ga = GeneticAlgorithm::new(
            RouletteWheelSelection::new(),
            UniformCrossover::new(),
            GaussianMutation::new(MUTATION_RATE, MUTATION_STRENGTH),
        );

        Evolution {
            population,
            ga,
            age: 0,
        }
    }

    pub(crate) fn step(&mut self, rng: &mut dyn RngCore) {
        self.process_brains();
        self.process_movement();
        self.process_finished_games();
        self.process_evolution(rng);

        self.age += 1;
    }

    fn process_brains(&mut self) {
        todo!("Implement process brains")
    }

    fn process_movement(&mut self) {
        todo!("Implement process movement")
    }

    fn process_finished_games(&mut self) {
        todo!("Implement process finished games")
    }

    fn process_evolution(&mut self, rng: &mut dyn RngCore) {
        todo!("Implement process evolution")
    }

    /// Step until end of current generation
    pub(crate) fn train(&mut self, rng: &mut dyn RngCore) {
        // TODO: Add progress bar
        while self.age < GENERATION_LENGTH {
            self.step(rng);
        }
    }

    pub(crate) fn save(&self) {
        todo!("Implement save")
    }

    pub(crate) fn best_individual(&self) -> &Snake {
        self.population
            .iter()
            .max_by_key(|s| OrderedFloat::from(s.fitness()))
            .unwrap()
    }

    pub(crate) fn age(&self) -> usize {
        self.age
    }
}
