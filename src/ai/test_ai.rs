use crate::vector_tools::vector_accuracy;

use super::ai::AI;

pub fn test_ai(ai: AI, test_data: Vec<f64>, test_data_output: Vec<f64>) -> f64 {
    let mut predictions = vec![];

    print!("<");

    for item in test_data {
        predictions.push(ai.predict(item));
        print!("=");
    }

    print!(">\n");

    let accuracy = vector_accuracy(predictions, test_data_output, 20.0);

    accuracy
}
