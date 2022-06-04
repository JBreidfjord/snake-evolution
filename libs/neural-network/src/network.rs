use crate::*;

#[derive(Debug, Clone)]
pub struct Network {
    layers: Vec<Layer>,
}

impl Network {
    pub fn random(rng: &mut dyn rand::RngCore, layers: &[LayerTopology]) -> Network {
        assert!(layers.len() > 1);

        let layers = layers
            .windows(2)
            .map(|layers| {
                Layer::random(
                    rng,
                    layers[0].neurons,
                    layers[1].neurons,
                    layers[1].activation,
                )
            })
            .collect();

        Network { layers }
    }

    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propagate(inputs))
    }

    pub fn weights(&self) -> impl Iterator<Item = f32> + '_ {
        use std::iter::once;

        self.layers
            .iter()
            .flat_map(|layer| layer.neurons.iter())
            .flat_map(|neuron| once(&neuron.bias).chain(&neuron.weights))
            .cloned()
    }

    pub fn from_weights(
        layers: &[LayerTopology],
        weights: impl IntoIterator<Item = f32>,
    ) -> Network {
        assert!(layers.len() > 1);

        let mut weights = weights.into_iter();
        let layers = layers
            .windows(2)
            .map(|layers| {
                Layer::from_weights(
                    layers[0].neurons,
                    layers[1].neurons,
                    &mut weights,
                    layers[1].activation,
                )
            })
            .collect();

        if weights.next().is_some() {
            panic!("Too many weights!");
        }

        Network { layers }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    mod random {
        use super::*;

        #[test]
        fn test() {
            let mut rng = ChaCha8Rng::from_seed(Default::default());
            let network = Network::random(
                &mut rng,
                &[
                    LayerTopology {
                        neurons: 2,
                        activation: Activation::None,
                    },
                    LayerTopology {
                        neurons: 2,
                        activation: Activation::None,
                    },
                ],
            );

            assert_eq!(network.layers.len(), 1);
            assert_eq!(network.layers[0].neurons.len(), 2);

            assert_relative_eq!(network.layers[0].neurons[0].bias, 0.8181262);
            assert_relative_eq!(
                network.layers[0].neurons[0].weights.as_slice(),
                [-0.6255188, 0.67383957].as_ref()
            );

            assert_relative_eq!(network.layers[0].neurons[1].bias, -0.53516835);
            assert_relative_eq!(
                network.layers[0].neurons[1].weights.as_slice(),
                [0.26284897, 0.5238807].as_ref()
            );
        }
    }

    mod propagate {
        use super::*;

        #[test]
        fn test() {
            let network = Network {
                layers: vec![
                    Layer {
                        neurons: vec![
                            Neuron {
                                weights: vec![0.25, 0.75],
                                bias: 0.0,
                            },
                            Neuron {
                                weights: vec![0.5, 0.5],
                                bias: 0.5,
                            },
                        ],
                        activation: Activation::None,
                    },
                    Layer {
                        neurons: vec![
                            Neuron {
                                weights: vec![0.25, 0.75],
                                bias: 0.0,
                            },
                            Neuron {
                                weights: vec![0.5, 0.5],
                                bias: 0.5,
                            },
                        ],
                        activation: Activation::None,
                    },
                ],
            };

            let prop = network.propagate(vec![0.3, 0.6]);
            assert_relative_eq!(prop.as_slice(), [0.84375, 1.2375].as_ref());
        }

        #[test]
        fn test_activation_functions() {
            let network = Network {
                layers: vec![
                    Layer {
                        neurons: vec![
                            Neuron {
                                weights: vec![0.25, 0.75],
                                bias: 0.0,
                            },
                            Neuron {
                                weights: vec![0.5, 0.5],
                                bias: 0.5,
                            },
                        ],
                        activation: Activation::ReLU,
                    },
                    Layer {
                        neurons: vec![
                            Neuron {
                                weights: vec![0.25, 0.75],
                                bias: 0.0,
                            },
                            Neuron {
                                weights: vec![0.5, 0.5],
                                bias: 0.5,
                            },
                        ],
                        activation: Activation::Softmax,
                    },
                ],
            };

            let prop = network.propagate(vec![0.3, 0.6]);
            assert_relative_eq!(prop.iter().sum::<f32>(), 1.0);
            assert_relative_eq!(prop.as_slice(), [0.4028149, 0.5971851].as_ref());
        }

        #[test]
        fn test_random_with_activation_functions() {
            let mut rng = ChaCha8Rng::from_seed(Default::default());
            let topology = [
                LayerTopology {
                    neurons: 6,
                    activation: Activation::ReLU,
                },
                LayerTopology {
                    neurons: 4,
                    activation: Activation::ReLU,
                },
                LayerTopology {
                    neurons: 4,
                    activation: Activation::ReLU,
                },
                LayerTopology {
                    neurons: 4,
                    activation: Activation::Softmax,
                },
            ];
            let network = Network::random(&mut rng, &topology);
            let prop = network.propagate(vec![0.5, 1.0, 0.0, -0.5]);
            assert_relative_eq!(prop.iter().sum::<f32>(), 1.0);
        }
    }

    mod from_weights {
        use super::*;
        use approx::assert_relative_eq;

        #[test]
        fn test() {
            let layers = &[
                LayerTopology {
                    neurons: 3,
                    activation: Activation::None,
                },
                LayerTopology {
                    neurons: 2,
                    activation: Activation::None,
                },
            ];
            let weights = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];
            let network = Network::from_weights(layers, weights.clone());
            let actual: Vec<_> = network.weights().collect();

            assert_relative_eq!(actual.as_slice(), weights.as_slice());
        }
    }
}
