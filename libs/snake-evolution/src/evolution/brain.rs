use rand::prelude::*;

use lib_genetic_algorithm::Chromosome;
use lib_neural_network::{Activation, LayerTopology, Network};

#[derive(Clone)]
pub(crate) struct Brain {
    pub(crate) nn: Network,
}

impl Brain {
    pub(crate) fn random(rng: &mut dyn RngCore) -> Brain {
        Brain {
            nn: Network::random(rng, &Self::topology()),
        }
    }

    fn topology() -> [LayerTopology; 4] {
        [
            LayerTopology {
                neurons: 24,
                activation: Activation::None,
            },
            LayerTopology {
                neurons: 18,
                activation: Activation::None,
            },
            LayerTopology {
                neurons: 18,
                activation: Activation::None,
            },
            LayerTopology {
                neurons: 4,
                activation: Activation::Softmax,
            },
        ]
    }

    pub(crate) fn as_chromosome(&self) -> Chromosome {
        self.nn.weights().collect()
    }

    pub(crate) fn from_chromosome(chromosome: Chromosome) -> Brain {
        Brain {
            nn: Network::from_weights(&Self::topology(), chromosome),
        }
    }
}
