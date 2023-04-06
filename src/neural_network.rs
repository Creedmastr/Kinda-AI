use crate::vector_tools;

#[derive(Debug, Default, Clone)]
pub struct NeuralNetwork {
    results: Vec<f64>,
    weight: Vec<f64>,
    accuracy: f64,
    max_value: f64
}

pub trait ToNeural {
    fn to_neural_network(&self) -> NeuralNetwork;
}

impl ToNeural for (Vec<f64>, Vec<f64>, f64, f64) {
    fn to_neural_network(&self) -> NeuralNetwork {
        let result = NeuralNetwork {
            results: self.0.clone(),
            weight: self.1.clone(),
            accuracy: self.2,
            max_value: self.3,
        };

        result
    }
}

impl NeuralNetwork {
    pub fn predict(&self, value: f64) -> f64 {
        let value_index = vector_tools::closest_value_index(self.results.clone(), value);
        let value_weights = self.weight[value_index];
        let weighted_value = value * value_weights;

        weighted_value
    }
}