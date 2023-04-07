use crate::tools::{tools::accuracy, tools::is_closer_to};

use super::tools::average;

// Various tools for manipulating Vector as it is the most used type in the code

pub fn vec_average(values: &Vec<f64>) -> f64 {
    let sum: f64 = values.iter().sum();
    let avg: f64 = sum / values.len() as f64;
    avg
}

pub fn closest_value_index(vec: Vec<f64>, value: f64) -> usize {
    let mut current: usize = 0;
    let mut result: usize = 0;
    let mut index = 0;

    for _item in vec.clone() {
        if is_closer_to(vec[index], value, current as f64) {
            current = value as usize;
            result = index as usize;
        }

        index += 1;
    }

    result
}

// Accuracy of a vector over another vector(for each same index compared to another)
pub fn vector_accuracy(
    vec1: Vec<f64>,
    vec2: Vec<f64>,
    max_value: f64,
    is_average: bool,
) -> Vec<f64> {
    if vec1.len() != vec2.len() {
        panic!("Arguments don't have the same number of elements")
    }

    let mut buffer = 0;
    let mut result: Vec<f64> = vec![];

    for _i in vec1.clone() {
        let item_to_add = accuracy(vec1[buffer], vec2[buffer], max_value);

        result.push(item_to_add);

        buffer += 1;
    }

    if is_average == true {
        vec![vec_average(&result)]
    } else {
        result
    }
}

// Average of two vectors (from items with the same index)
pub fn two_vector_average(vec1: Vec<f64>, vec2: Vec<f64>) -> Vec<f64> {
    if vec1.len() != vec2.len() {
        panic!("Arguments don't have the same number of elements")
    }

    let mut buffer = 0;
    let mut result: Vec<f64> = vec![];

    for _i in vec1.clone() {
        let item_to_add = average(vec1[buffer], vec2[buffer]);

        result.push(item_to_add);

        buffer += 1;
    }

    result
}

fn cosine_similarity(a: &[f64], b: &[f64]) -> f64 {
    let dot_product = a.iter().zip(b).map(|(&x, &y)| x * y).sum::<f64>();
    let magnitude_a = (a.iter().map(|x| x.powi(2)).sum::<f64>()).sqrt();
    let magnitude_b = (b.iter().map(|x| x.powi(2)).sum::<f64>()).sqrt();

    if magnitude_a == 0.0 || magnitude_b == 0.0 {
        0.0
    } else {
        dot_product / (magnitude_a * magnitude_b)
    }
}


pub fn vec_match(vec: Vec<Vec<f64>>, item: Vec<f64>) -> usize {
    let mut index: usize = 0;
    let mut current_index: usize = 0;

    let mut previous_similarity: f64 = 0.0;

    for _i in &vec {
        let current_similarity = cosine_similarity(&vec[index], &item);

        if  current_similarity > previous_similarity {
            previous_similarity = current_similarity;
            current_index = index
        };

        index += 1;
    }

    current_index
}
