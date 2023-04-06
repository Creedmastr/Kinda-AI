use crate::vector_tools;

#[derive(Debug, Default, Clone)]
pub struct AI {
    results: Vec<f64>,
    weight: Vec<f64>,
    accuracy: f64,
    max_value: f64,
}

pub trait ToAI {
    fn to_ai(&self) -> AI;
}

impl ToAI for (Vec<f64>, Vec<f64>, f64, f64) {
    fn to_ai(&self) -> AI {
        let result = AI {
            results: self.0.clone(),
            weight: self.1.clone(),
            accuracy: self.2,
            max_value: self.3,
        };

        result
    }
}

impl AI {
    pub fn predict(&self, value: f64) -> f64 {
        let value_index = vector_tools::closest_value_index(self.results.clone(), value);
        let value_weights = self.weight[value_index];
        let weighted_value;

        if value * value_weights > self.max_value {
            weighted_value = self.max_value;
        } else {
            weighted_value = value * value_weights;
        }

        weighted_value
    }
}

fn test_split(mut vec: Vec<f64>, split_value: f64) -> (Vec<f64>, Vec<f64>) {
    let train_data = vec.split_off((vec.len() as f64 * (1.0 - split_value)) as usize);

    (train_data, vec)
}
