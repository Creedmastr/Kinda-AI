use crate::tools::vector_tools::{vector_vector_accuracy, vec_average};

use super::ai::AI;

#[derive(Debug)]
pub enum TestResult {
    Float(f64),
    Vec2D(Vec<Vec<f64>>),
}

pub fn test_ai(
    ai: AI,
    test_data: Vec<Vec<f64>>,
    test_data_output: Vec<Vec<f64>>,
    is_averaged: bool
) -> TestResult {
    let mut predictions = vec![];

    // Uses arrows etc. for showing progress (on low-end machines or large data)
    print!("<");

    for item in test_data {
        predictions.push(ai.predict(item));
        print!("=");
    }

    print!(">\n");

    // Accuracy
    if is_averaged == true {
        TestResult::Float(vec_average(vector_vector_accuracy(predictions, test_data_output, 20.0, is_averaged)[0].as_ref()))
    } else {
        TestResult::Vec2D(vector_vector_accuracy(predictions, test_data_output, 20.0, false))
    }
}

