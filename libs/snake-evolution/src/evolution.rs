use ordered_float::OrderedFloat;
use rand::prelude::*;

use lib_genetic_algorithm::{
    GaussianMutation, GeneticAlgorithm, RouletteWheelSelection, UniformCrossover,
};

use snake::Snake;
use snake_individual::SnakeIndividual;

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

    pub(crate) fn step(&mut self) {
        self.process_movement();
        self.age += 1;
    }

    fn process_movement(&mut self) {
        for snake in &mut self.population {
            snake.make_move();
        }
    }

    fn process_evolution(&mut self, rng: &mut dyn RngCore) {
        self.age = 0;

        // Transform Vec<Snake> into Vec<SnakeIndividual>
        let current_population: Vec<_> = self
            .population
            .iter()
            .map(SnakeIndividual::from_snake)
            .collect();

        // Evolve the population
        let evolved_population = self.ga.step(rng, &current_population);

        // Transform Vec<SnakeIndividual> into Vec<Creature>
        self.population = evolved_population
            .into_iter()
            .map(|individual| individual.into_snake())
            .collect();
    }

    /// Step until end of current generation
    pub(crate) fn train(&mut self, rng: &mut dyn RngCore) {
        // TODO: Add progress bar
        while self.age < GENERATION_LENGTH {
            self.step();
        }
        self.process_evolution(rng);
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

    fn worst_individual(&self) -> &Snake {
        self.population
            .iter()
            .min_by_key(|s| OrderedFloat::from(s.fitness()))
            .unwrap()
    }

    pub(crate) fn age(&self) -> usize {
        self.age
    }

    pub(crate) fn replay(&self, which: &str) {
        if which != "best" || which != "worst" {
            return;
        }

        let snake = if which == "best" {
            self.best_individual()
        } else {
            self.worst_individual()
        };

        for display in &snake.history {
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            println!("{}", display);
        }
    }
}
