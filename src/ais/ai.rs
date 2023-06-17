use std::fs::File;
use std::io::BufReader;

use crate::vector_tools;
use serde;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AI {
    results: Vec<Vec<f64>>,
    weight: Vec<Vec<f64>>,
    max_value: f64,
}

pub trait ToAI {
    fn to_ai(&self) -> AI;
}

impl ToAI for (Vec<Vec<f64>>, Vec<Vec<f64>>, f64) {
    fn to_ai(&self) -> AI {
        let result = AI {
            results: self.0.clone(),
            weight: self.1.clone(),
            max_value: self.2,
        };

        result
    }
}

impl AI {
    // Make a function using an AI model
    pub fn predict(&self, value_vec: Vec<f64>) -> Vec<f64> {
        let value_index =
            vector_tools::index_of_closest_vec_in_vec(self.results.clone(), value_vec.clone());
        let value_weights = self.weight[value_index].clone();
        let mut prediction = vec![];
        let mut buffer = 0;

        for item in value_vec {
            if item * value_weights[buffer] > self.max_value.clone() {
                prediction.push(self.max_value)
            } else {
                prediction.push(item * value_weights[buffer])
            }

            buffer += 1;
        }

        prediction
    }

    // Save it to the disk
    pub fn save(&self, filename: &str) {
        let file = File::create(filename).unwrap();
        serde_json::to_writer(file, self).unwrap();
    }

    pub fn load_ai_from_file(filename: &str) -> AI {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let ai = serde_json::from_reader(reader).unwrap();
        ai
    }
}
