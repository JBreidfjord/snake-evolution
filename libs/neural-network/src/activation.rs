use std::f32::consts::E;

#[derive(Debug, Clone, Copy)]
pub enum Activation {
    ReLU,
    Sigmoid,
    Softmax,
    None,
}

impl Activation {
    pub(crate) fn apply(&self, inputs: Vec<f32>) -> Vec<f32> {
        let f = match *self {
            Activation::ReLU => relu,
            Activation::Sigmoid => sigmoid,
            Activation::Softmax => softmax,
            Activation::None => none,
        };

        f(inputs)
    }
}

fn relu(inputs: Vec<f32>) -> Vec<f32> {
    inputs.iter().map(|x| x.max(0.0)).collect()
}

fn sigmoid(inputs: Vec<f32>) -> Vec<f32> {
    inputs.iter().map(|x| 1.0 / (1.0 + E.powf(-x))).collect()
}

fn softmax(inputs: Vec<f32>) -> Vec<f32> {
    let exponential_inputs: Vec<f32> = inputs.iter().map(|x| x.exp()).collect();
    let sum: f32 = exponential_inputs.iter().sum();
    exponential_inputs.iter().map(|x| x / sum).collect()
}

fn none(inputs: Vec<f32>) -> Vec<f32> {
    inputs
}
