#![allow(non_snake_case)]
#![allow(dead_code)]

mod vector_tools;
mod tools;
mod neural;

use std::vec;
use neural::train_ai::*;
use tools::*;

fn main() {
    let first_trimester_avg = vec![15.5, 16.6, 19.2, 19.8, 12.2, 14.4, 18.2, 7.1, 10.2, 2.0, 0.1, 1.2, 16.4, 15.7, 13.2, 12.1, 11.9, 12.0];

    let third_trimester_avg = vec![16.4, 16.5, 19.3, 19.8, 14.1, 15.02, 17.9, 10.2, 11.5, 4.8, 1.2, 6.5, 15.6, 16.9, 14.4, 13.2, 12.9, 11.0];

    println!(
        "{:#?}",
        train_ai(
            first_trimester_avg,
            third_trimester_avg,
            20.0,
            0.00001,
        )
    );
}
