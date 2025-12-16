/// Calculates the Z-score for a binomial distribution.
///
/// # Arguments
/// * `trials` - The total number of sessions/trials.
/// * `successes` - The number of times the target event occurred (e.g., selecting Choice A).
///
/// # Returns
/// The Z-score, which represents how many standard deviations the result is from the mean.
/// A Z-score with absolute value > 1.96 indicates statistical significance at p < 0.05.
pub fn calculate_z_score(trials: usize, successes: usize) -> f64 {
    let n = trials as f64;
    let x = successes as f64;
    // We assume a 50/50 probability for binary choice in a random system
    let p = 0.5;

    let mean = n * p;
    let std_dev = (n * p * (1.0 - p)).sqrt();

    // Avoid division by zero
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
