pub fn calculate_z_score(trials: usize, successes: usize) -> f64 {
    let n = trials as f64;
    let x = successes as f64;
    let p = 0.5;

    let mean = n * p;
    let std_dev = (n * p * (1.0 - p)).sqrt();

    if std_dev == 0.0 {
        return 0.0;
    }

    (x - mean) / std_dev
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_z_score_exact_mean() {
        // 100 trials, 50 successes -> Z should be 0
        let z = calculate_z_score(100, 50);
        assert!((z - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_z_score_deviation() {
        // 100 trials, 60 successes
        // mean = 50, std_dev = 5
        // z = (60 - 50) / 5 = 2.0
        let z = calculate_z_score(100, 60);
        assert!((z - 2.0).abs() < 1e-10);
    }
}
