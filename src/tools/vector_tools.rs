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

// Accuracy of a vector over another
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

// Accuracy of a vector composed of vectors over another vector(for each same index compared to another)
pub fn vector_vector_accuracy(vec1: Vec<Vec<f64>>, vec2: Vec<Vec<f64>>, max_value: f64, is_averaged: bool) -> Vec<Vec<f64>> {
    let mut buffer = 0;
    let mut result: Vec<Vec<f64>> = vec![];

    for _i in vec1.clone() {
        let item_to_add = vector_accuracy(vec1[buffer].clone(), vec2[buffer].clone(), max_value, is_averaged);

        result.push(item_to_add);

        buffer += 1;
    }

    result
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

// Give the ressemblance percentage between two vectors
pub fn cosine_similarity(a: &[f64], b: &[f64]) -> f64 {
    let dot_product = a.iter().zip(b).map(|(&x, &y)| x * y).sum::<f64>();
    let magnitude_a = (a.iter().map(|x| x.powi(2)).sum::<f64>()).sqrt();
    let magnitude_b = (b.iter().map(|x| x.powi(2)).sum::<f64>()).sqrt();

    if magnitude_a == 0.0 || magnitude_b == 0.0 {
        0.0
    } else {
        dot_product / (magnitude_a * magnitude_b)
    }
}

// Give the index of the closest value (a vector) in a vector of vector (Vec<Vecf64>>)
pub fn index_of_closest_vec_in_vec(vec: Vec<Vec<f64>>, item: Vec<f64>) -> usize {
    let mut index: usize = 0;
    let mut current_index: usize = 0;

    let mut previous_similarity: f64 = 0.0;

    for _i in &vec {
        let current_similarity = cosine_similarity(&vec[index], &item);

        if current_similarity > previous_similarity {
            previous_similarity = current_similarity;
            current_index = index
        };

        index += 1;
    }

    current_index
}

// A trait to be able to return two different types in the test_ai() function
pub trait ToVectorOfVector {
    fn to_vector_of_vector(self) -> Vec<Vec<f64>>;
}

impl ToVectorOfVector for Vec<f64> {
    fn to_vector_of_vector(self) -> Vec<Vec<f64>> {
        let mut result: Vec<Vec<f64>> = vec![];

        for item in self {
            result.push(vec![item]);
        }

        result
    }
}