pub fn average(number1: f64, number2: f64) -> f64 {
    (number1 + number2) / 2.0
}

pub fn accuracy(actual: f64, expected: f64, max_value: f64) -> f64 {
    let error = (actual - expected).abs();
    let accuracy = 100.0 * (1.0 - error / max_value);
    accuracy
}

pub fn is_closer_to(f: f64, a: f64, b: f64) -> bool {
    let diff_a = (f - a).abs();
    let diff_b = (f - b).abs();
    diff_a < diff_b
}
