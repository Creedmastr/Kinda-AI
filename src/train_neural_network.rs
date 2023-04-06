use crate::{vector_tools::vector_accuracy, tools::is_closer_to};

pub fn train_neural_network(input_vec: Vec<f64>, output_vec: Vec<f64>, max_value: f64, generation_number: f64) -> (Vec<f64>, Vec<f64>, f64, f64) {
    let mut current_result: f64;
    let mut weight: f64;
    let mut results_vec: Vec<f64> = vec![];
    let mut weight_vec: Vec<f64> = vec![];
    let mut current_result_weight: f64 = 0.0;
    let mut buffer = 0;

    current_result = 0.0;

    for _item in input_vec.clone() {
        weight = 0.4;
        loop {
            let mut weighed_input: f64 = input_vec[buffer] * weight;
            weight += generation_number;
            
            if weighed_input > max_value {
                weighed_input = max_value;
            }
    
            if is_closer_to(output_vec[buffer], weighed_input, current_result) {
                current_result_weight = weight;
                current_result = weighed_input;
            }

            if weight > 2.0 {
                break
            }
        }
        weight_vec.push(current_result_weight);
        results_vec.push(current_result);
        current_result = 0.0;
        
        // current_result = 0.0;
        buffer += 1;
    }

    (results_vec.clone(), weight_vec, vector_accuracy(results_vec, output_vec, 20.0), max_value)
}