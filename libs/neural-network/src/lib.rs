use rand::Rng;

pub use self::{activation::*, layer_topology::*, network::*};
use self::{layer::*, neuron::*};

mod activation;
mod layer;
mod layer_topology;
mod network;
mod neuron;
