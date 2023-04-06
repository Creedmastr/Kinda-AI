#![allow(non_snake_case)]

use std::any::Any;

fn accuracy(actual: f64, expected: f64, max_value: f64) -> f64 {
    let error = (actual - expected).abs();
    let accuracy = 100.0 * (1.0 - error / max_value);
    accuracy
}


fn is_closer_to(f: f64, a: f64, b: f64) -> bool {
    let diff_a = (f - a).abs();
    let diff_b = (f - b).abs();
    diff_a < diff_b
}

struct NeuralNetwork {
    output: f64,
}

fn train_neural_network(input: f64, output: f64) -> (f64, f64) {
    let mut result: f64 = 0.0;
    let mut current_result: f64;
    let mut weight: f64 = 0.0;

    current_result = 0.0;

    loop {
        let weighed_input: f64 = input * weight;
        println!("{}", weighed_input);

        if is_closer_to(output, weighed_input, current_result) {
            current_result = weighed_input;
        }

        match weight >= 1.0 {
            true => break,
            false => weight += 0.1,
        }
    }

    result = current_result;

    (result, accuracy(result, output, 20.0))
}

fn average(number1: f64, number2: f64) -> f64 {
    (number1 + number2) / 2.0
}

fn main() {
    let first_trimester_avg = 16.4;
    let second_trimester_avg = 16.2;

    let third_trimester_avg = 17.2;

    println!(
        "{:#?}",
        train_neural_network(
            average(first_trimester_avg, second_trimester_avg),
            third_trimester_avg
        )
    );
}
