use crate::tools::tools::is_closer_to;

pub fn train_ai(
    input_vec: Vec<f64>,
    output_vec: Vec<f64>,
    max_value: f64,
    generation_number: f64,
) -> (Vec<f64>, Vec<f64>, f64) {
    // Initialise all the mutable variables
    let mut current_result: f64 = 0.0;
    let mut weight: f64;
    let mut results_vec: Vec<f64> = vec![];
    let mut weight_vec: Vec<f64> = vec![];
    let mut current_result_weight: f64 = 0.0;
    let mut buffer = 0;

    print!("<");

    for _item in input_vec.clone() {
        // Reset weight
        weight = 0.0;

        // Finds the exact weights to get from each element to their equivalent
        loop {
            let mut weighed_input: f64 = input_vec[buffer] * weight;
            weight += generation_number;

            if weighed_input > max_value {
                // If exceeds the maximum value reset it
                weighed_input = max_value;
            }

            // If the weighed number is closer to the real number, then set the current 'best' score to it
            if is_closer_to(output_vec[buffer], weighed_input, current_result) {
                current_result_weight = weight;
                current_result = weighed_input;
            }

            // Sets a limit to the loop
            if weight > 5.0 {
                break;
            }
        }
        // Gets the weight and the results vector with the same index (important for later)
        weight_vec.push(current_result_weight);
        results_vec.push(current_result);
        current_result = 0.0; // resets current_result

        buffer += 1; // Ups the buffer
        print!("="); // Show the progress
    }

    print!(">\n");

    (results_vec.clone(), weight_vec, max_value)
}
