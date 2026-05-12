/// Simpson's rule for numerical integration
pub fn quad<F>(f: F, a: f64, b: f64, n: usize) -> f64
where
    F: Fn(f64) -> f64,
{
    assert!(n > 0, "number of subdivisions must be positive");
    assert!(n % 2 == 0, "Simpson's rule requires an even number of subdivisions");

    let h = (b - a) / n as f64;
    let mut sum = f(a) + f(b);

    for i in 1..n {
        let x = a + i as f64 * h;
        let weight = if i % 2 == 0 { 2.0 } else { 4.0 };
        sum += weight * f(x);
    }

    h * sum / 3.0
}

#[cfg(test)]
mod tests {
    use super::quad;

    #[test]
    fn integrates_sine_on_zero_to_pi() {
        let result = quad(|x| f64::sin(x), 0.0, std::f64::consts::PI, 1000);

        assert!((result - 2.0).abs() < 2.0e-12);
    }
}
