use crate::tools::is_closer_to;

pub fn train_ai(
    input_vec: Vec<f64>,
    output_vec: Vec<f64>,
    max_value: f64,
    generation_number: f64,
) -> (Vec<f64>, Vec<f64>, f64) {
    let mut current_result: f64;
    let mut weight: f64;
    let mut results_vec: Vec<f64> = vec![];
    let mut weight_vec: Vec<f64> = vec![];
    let mut current_result_weight: f64 = 0.0;
    let mut buffer = 0;

    current_result = 0.0;

    print!("<");

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

            if weight > 5.0 {
                break;
            }
        }
        weight_vec.push(current_result_weight);
        results_vec.push(current_result);
        current_result = 0.0;

        // current_result = 0.0;
        buffer += 1;
        print!("=");
    }

    print!(">\n");

    (results_vec.clone(), weight_vec, max_value)
}
