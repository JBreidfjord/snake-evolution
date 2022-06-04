use crate::*;

#[derive(Debug, Clone)]
pub(crate) struct Neuron {
    pub(crate) weights: Vec<f32>,
    pub(crate) bias: f32,
}

impl Neuron {
    pub fn random(rng: &mut dyn rand::RngCore, output_size: usize) -> Neuron {
        let weights = (0..output_size)
            .map(|_| rng.gen_range(-1.0..=1.0))
            .collect();

        let bias = rng.gen_range(-1.0..=1.0);

        Neuron { weights, bias }
    }

    pub(crate) fn propagate(&self, inputs: &[f32]) -> f32 {
        inputs
            .iter()
            .zip(&self.weights)
            .map(|(input, weight)| weight * input)
            .sum::<f32>()
            + self.bias
    }

    pub fn from_weights(output_neurons: usize, weights: &mut dyn Iterator<Item = f32>) -> Neuron {
        let bias = weights.next().expect("Not enough weights!");
        let weights = (0..output_neurons)
            .map(|_| weights.next().expect("Not enough weights!"))
            .collect();

        Neuron { weights, bias }
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
            let neuron = Neuron::random(&mut rng, 2);

            assert_relative_eq!(neuron.bias, 0.8181262);
            assert_relative_eq!(neuron.weights.as_slice(), [-0.6255188, 0.67383957].as_ref());
        }
    }

    mod propagate {
        use super::*;
        use approx::assert_relative_eq;

        #[test]
        fn test() {
            let neuron = Neuron {
                weights: vec![0.5, 0.5],
                bias: 0.5,
            };

            assert_relative_eq!(neuron.propagate(&[-10.0, -10.0]), -9.5);

            assert_relative_eq!(neuron.propagate(&[1.0, 0.5]), 1.25);
        }
    }
}
