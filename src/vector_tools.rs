use crate::{
    average,
    tools::{accuracy, is_closer_to},
};

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
pub fn vector_accuracy(vec1: Vec<f64>, vec2: Vec<f64>, max_value: f64) -> f64 {
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

    vec_average(&result)
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
