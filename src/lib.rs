#![allow(non_snake_case)]
#![allow(dead_code)]

mod ai;
mod tools;

use ai::train_ai::*;
use std::vec;
use tools::*;

use crate::ai::{ai::ToAI, test_ai::test_ai};

pub fn run() {
    let first_trimester_avg = vec![
        15.5, 16.6, 19.2, 19.8, 12.2, 14.4, 18.2, 7.1, 10.2, 2.0, 0.1, 1.2,
    ];
    let third_trimester_avg = vec![
        16.4, 16.5, 19.3, 19.8, 14.1, 15.02, 17.9, 10.2, 11.5, 4.8, 1.2, 6.5,
    ];

    let test_data_first = vec![16.4, 15.7, 13.2, 12.1, 11.9, 12.0];
    let test_data_second = vec![15.6, 16.9, 14.4, 13.2, 12.9, 11.0];

    println!(
        "{:#?}",
        test_ai(
            train_ai(first_trimester_avg, third_trimester_avg, 20.0, 0.00001).to_ai(),
            test_data_first,
            test_data_second,
            false
        )
    );
}

// If you need to save the AI model
/*train_ai(first_trimester_avg, third_trimester_avg, 20.0, 0.001)
    .to_ai()
    .save("./ai");

println!(
    "{:#?}",
    test_ai(
        AI::load_ai_from_file("./ai"),
        test_data_first,
        test_data_second
    )
); */
