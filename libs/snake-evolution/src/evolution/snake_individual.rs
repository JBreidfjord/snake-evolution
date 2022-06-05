use rand::prelude::*;

use lib_genetic_algorithm::{Chromosome, Individual};

use crate::evolution::Snake;

pub(crate) struct SnakeIndividual {
    fitness: f32,
    chromosome: Chromosome,
}

impl Individual for SnakeIndividual {
    fn create(chromosome: Chromosome) -> SnakeIndividual {
        SnakeIndividual {
            fitness: 0.0,
            chromosome,
        }
    }

    fn chromosome(&self) -> &Chromosome {
        &self.chromosome
    }

    fn fitness(&self) -> f32 {
        self.fitness
    }
}

impl SnakeIndividual {
    pub(crate) fn from_snake(snake: &Snake) -> SnakeIndividual {
        SnakeIndividual {
            fitness: snake.fitness(),
            chromosome: snake.as_chromosome(),
        }
    }

    pub(crate) fn into_snake(self) -> Snake {
        Snake::from_chromosome(self.chromosome)
    }
}
