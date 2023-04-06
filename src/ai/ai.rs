use std::fs::File;
use std::io::BufReader;

use crate::vector_tools;
use serde;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AI {
    results: Vec<f64>,
    weight: Vec<f64>,
    max_value: f64,
}

pub trait ToAI {
    fn to_ai(&self) -> AI;
}

impl ToAI for (Vec<f64>, Vec<f64>, f64) {
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