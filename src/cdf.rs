use std::f32::consts::PI;

/// approximate erf function with a taylor series
fn erf(z: f32, degrees: u32) -> f32 {
    let mut sum = z;
    let mut factorial_n = 1;
    let degrees = degrees + 1;
    for n in 1..degrees {
        factorial_n *= n;

        // TODO: track a sign variable and compare performance
        let numerator = (i32::pow(-1, n) as f32) * z.powi(2 * (n as i32) + 1);
        let denominator = (factorial_n * (2 * n + 1)) as f32;
        sum += numerator / denominator;
    }

    (2.0 / PI.sqrt()) * sum
}

#[cfg(test)]
mod tests {
    use super::*;

    /// test that erf with 0 degrees works as expected
    #[test]
    fn erf_zero_degree() {
        assert!(erf(0.0, 0) == 0.0);
        assert!(erf(1.0, 0) == (2.0 / PI.sqrt()));
        assert!(erf(2.0, 0) == (4.0 / PI.sqrt()));
        assert!(erf(3.0, 0) == (6.0 / PI.sqrt()));
    }

    /// test that erf with 1 degrees works as expected
    #[test]
    fn erf_one_degree() {
        assert!(erf(0.0, 1) == 0.0);
        assert!(erf(1.0, 1) == (2.0 / PI.sqrt()));
        assert!(erf(2.0, 1) == (4.0 / PI.sqrt()));
        assert!(erf(3.0, 1) == (6.0 / PI.sqrt()));
    }    
}