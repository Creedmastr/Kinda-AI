use crate::tools::tools::is_closer_to;

pub fn train_ai(
    input_vec: Vec<Vec<f64>>,
    output_vec: Vec<Vec<f64>>,
    max_value: f64,
    precision: f64,
) -> (Vec<Vec<f64>>, Vec<Vec<f64>>, f64) {
    let mut current_result: f64 = 0.0;
    let mut weight: f64;
    let mut results_vec: Vec<Vec<f64>> = vec![];
    let mut weight_vec: Vec<Vec<f64>> = vec![];
    let mut current_result_weight: f64 = 0.0;
    let mut inferior_buffer = 0;
    let mut superior_buffer = 0;

    print!("<");
    // Check for each vector in the vector 'collection'
    for item in input_vec {
        let mut current_weight_vec: Vec<f64> = vec![];
        let mut current_result_vec: Vec<f64> = vec![];

        // Check for each f64
        for &input in &item {
            weight = 0.0;

            loop {
                let mut weighed_input = input * weight;
                weight += precision;

                // Checks to avoid any errors (Rust errors and AI errors)
                if weighed_input > max_value {
                    weighed_input = max_value;
                }

                if superior_buffer >= output_vec.len() {
                    superior_buffer = output_vec.len();
                }

                if inferior_buffer >= item.len() {
                    inferior_buffer = output_vec.len();
                }

                if is_closer_to(
                    output_vec[superior_buffer][inferior_buffer],
                    weighed_input,
                    current_result,
                ) {
                    current_result_weight = weight;
                    current_result = weighed_input;
                }

                // Add a limit to the loop (of 5)
                if weight > 5.0 {
                    break;
                }
            }

            current_weight_vec.push(current_result_weight);
            current_result_vec.push(current_result);

            inferior_buffer += 1;
        }

        superior_buffer += 1;

        weight_vec.push(current_weight_vec);
        results_vec.push(current_result_vec);
        current_result = 0.0;

        inferior_buffer = 0;
        print!("=");
    }

    print!(">\n");

    (results_vec, weight_vec, max_value)
}
