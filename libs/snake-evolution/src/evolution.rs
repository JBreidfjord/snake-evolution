use rand::prelude::*;

use lib_genetic_algorithm::{
    GaussianMutation, GeneticAlgorithm, RouletteWheelSelection, UniformCrossover,
};

use snake::Snake;

mod body;
mod brain;
mod snake;
mod snake_individual;

const POPULATION_SIZE: usize = 2000;
const GAME_GRID_SIZE: usize = 10;
const MUTATION_RATE: f32 = 0.15;
const MUTATION_STRENGTH: f32 = 0.3;

pub(crate) struct Evolution {
    population: Vec<Snake>,
    ga: GeneticAlgorithm<RouletteWheelSelection>,
    age: usize,
}

impl Evolution {
    pub(crate) fn random(rng: &mut dyn RngCore) -> Evolution {
        let population: Vec<Snake> = (0..POPULATION_SIZE)
            .map(|_| Snake::random(rng, GAME_GRID_SIZE))
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
}
