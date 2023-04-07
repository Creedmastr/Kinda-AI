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
        test_ai(ai, test_data_first, test_data_second, false),
    ); 
}