#![allow(non_snake_case)]
#![allow(dead_code)]

mod train_neural_network;
mod vector_tools;
mod tools;
mod neural_network;

use std::vec;
use train_neural_network::*;
use vector_tools::*;
use tools::*;

use crate::neural_network::ToNeural;

fn main() {
    let first_trimester_avg = vec![15.5, 16.6, 19.2, 19.8, 12.2, 14.4, 18.2, 7.1];
    let second_trimester_avg = vec![16.2, 16.3, 19.4, 19.8, 13.2, 15.2, 18.35, 9.2];

    let third_trimester_avg = vec![16.4, 16.5, 19.3, 19.8, 14.1, 15.02, 17.9, 10.2];

    println!(
        "{:#?}",
        train_neural_network(
            two_vector_average(first_trimester_avg, second_trimester_avg),
            third_trimester_avg,
            20.0,
            0.000001
        )
    );
}
