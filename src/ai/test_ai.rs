use crate::{vector_tools::vector_accuracy, tools::vector_tools::vector_vector_accuracy};

use super::ai::AI;

pub fn test_ai(
    ai: AI,
    test_data: Vec<Vec<f64>>,
    test_data_output: Vec<Vec<f64>>,
) -> Vec<Vec<f64>> {
    let mut predictions = vec![];
    let mut accuracy: Vec<Vec<f64>> = vec![];

    // Uses arrows etc. for showing progress (on low-end machines or large data)
    print!("<");

    for item in test_data {
        predictions.push(ai.predict(item));
        print!("=");
    }

    print!(">\n");

    vector_vector_accuracy(predictions, test_data_output, 20.0) // Accuracy
}
