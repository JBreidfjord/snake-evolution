use lib_neural_network::{LayerTopology, Network};
use rand::prelude::*;

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
            LayerTopology { neurons: 24 },
            LayerTopology { neurons: 18 },
            LayerTopology { neurons: 18 },
            LayerTopology { neurons: 4 },
        ]
    }
}
