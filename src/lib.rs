#![allow(non_snake_case)]
#![allow(dead_code)]

pub mod ais;
pub mod tools;

pub use ais::train_ai::*;
pub use std::vec;
pub use tools::*;
pub use vector_tools::*;
pub use ais::ai::*;
pub use ais::train_ai::*;
pub use ais::test_ai::*;

pub use crate::{ais::{ai::ToAI, test_ai::test_ai}, tools::vector_tools::{ToVectorOfVector, ToCorrectAmount}};

pub fn run() {
    let first_trimester_avg = vec![
        vec![16.4, 12.2], vec![18.4, 15.5], vec![19.2, 18.7],vec![12.4, 11.2], vec![19.4, 18.5], vec![7.2, 9.7]
    ];
    let third_trimester_avg = vec![
        vec![16.5, 12.75], vec![19.4, 16.25], vec![18.7, 19.2], vec![14.2, 11.2], vec![19.4, 18.2], vec![11.6, 12.9]
    ];

    let test_data_first = vec![
        vec![18.2, 17.2], vec![19.3, 18.2], vec![16.4, 12.2]
    ];

    let test_data_second = vec![
        vec![19.2, 18.2], vec![18.9, 17.3], vec![16.5, 12.6]
    ];

    let ai = train_ai(first_trimester_avg, third_trimester_avg, 20.0, 0.0001).to_ai();

    println!(
        "{:#?}",
        // test_ai(ai, test_data_first, test_data_second, false),
        vec![0.6, 19.3, 19.3, 10.2].to_vector_of_vector().to_correct_amount(2)
    ); 
}