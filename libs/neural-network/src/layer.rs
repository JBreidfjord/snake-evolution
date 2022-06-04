use crate::*;

#[derive(Debug, Clone)]
pub(crate) struct Layer {
    pub(crate) neurons: Vec<Neuron>,
    pub(crate) activation: Activation,
}

impl Layer {
    pub fn random(
        rng: &mut dyn rand::RngCore,
        input_neurons: usize,
        output_neurons: usize,
        activation: Activation,
    ) -> Layer {
        let neurons = (0..output_neurons)
            .map(|_| Neuron::random(rng, input_neurons))
            .collect();

        Layer {
            neurons,
            activation,
        }
    }

    pub(crate) fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        let outputs = self
            .neurons
            .iter()
            .map(|neuron| neuron.propagate(&inputs))
            .collect();

        self.activation.apply(outputs)
    }

    pub fn from_weights(
        input_size: usize,
        output_size: usize,
        weights: &mut dyn Iterator<Item = f32>,
        activation: Activation,
    ) -> Layer {
        let neurons = (0..output_size)
            .map(|_| Neuron::from_weights(input_size, weights))
            .collect();

        Layer {
            neurons,
            activation,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod random {
        use super::*;
        use approx::assert_relative_eq;
        use rand::SeedableRng;
        use rand_chacha::ChaCha8Rng;

        #[test]
        fn test() {
            let mut rng = ChaCha8Rng::from_seed(Default::default());
            let layer = Layer::random(&mut rng, 2, 2, Activation::None);

            assert_relative_eq!(layer.neurons[0].bias, 0.8181262);
            assert_relative_eq!(
                layer.neurons[0].weights.as_slice(),
                [-0.6255188, 0.67383957].as_ref()
            );

            assert_relative_eq!(layer.neurons[1].bias, -0.53516835);
            assert_relative_eq!(
                layer.neurons[1].weights.as_slice(),
                [0.26284897, 0.5238807].as_ref()
            );
        }
    }

    mod propagate {
        use super::*;
        use approx::assert_relative_eq;

        #[test]
        fn test() {
            let layer = Layer {
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
            };

            let prop = layer.propagate(vec![0.3, 0.6]);
            assert_relative_eq!(prop.as_slice(), [0.525, 0.95].as_ref());
        }

        #[test]
        fn test_activation_functions() {
            let relu_layer = Layer {
                neurons: vec![
                    Neuron {
                        weights: vec![0.25, 0.75],
                        bias: 0.0,
                    },
                    Neuron {
                        weights: vec![0.5, 0.5],
                        bias: -0.5,
                    },
                ],
                activation: Activation::ReLU,
            };
            let prop = relu_layer.propagate(vec![0.3, 0.6]);
            assert_relative_eq!(prop.as_slice(), [0.525, 0.0].as_ref());

            let sigmoid_layer = Layer {
                neurons: vec![
                    Neuron {
                        weights: vec![0.25, 0.75],
                        bias: 1.0,
                    },
                    Neuron {
                        weights: vec![1.5, -1.0],
                        bias: -0.5,
                    },
                ],
                activation: Activation::Sigmoid,
            };
            let prop = sigmoid_layer.propagate(vec![0.3, 0.6]);
            assert_relative_eq!(prop.as_slice(), [0.82127357, 0.34298956].as_ref());

            let softmax_layer = Layer {
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
            };
            let prop = softmax_layer.propagate(vec![0.3, 0.6]);
            assert_relative_eq!(prop.iter().sum::<f32>(), 1.0);
            assert_relative_eq!(prop.as_slice(), [0.3953209, 0.60467905].as_ref());
        }
    }
}
