/// Gaussian quadrature for numerical integration
/// Note: this method uses a fixed 10-point Gauss-Legendre rule.
pub fn quad<F>(f: F, a: f64, b: f64) -> f64
where
    F: Fn(f64) -> f64,
{
    let w = [
        0.2955242247147529,
        0.2955242247147529,
        0.2692667193099963,
        0.2692667193099963,
        0.219086362515982,
        0.219086362515982,
        0.1494513491505806,
        0.1494513491505806,
        0.0666713443086881,
        0.0666713443086881,
    ];
    let x = [
        -0.1488743389816312,
        0.1488743389816312,
        -0.4333953941292472,
        0.4333953941292472,
        -0.6794095682990244,
        0.6794095682990244,
        -0.8650633666889845,
        0.8650633666889845,
        -0.9739065285171717,
        0.9739065285171717,
    ];
    let n = w.len();

    let mut sum = 0.0;
    for i in 0..n {
        let xi = 0.5 * (b - a) * x[i] + 0.5 * (a + b);
        sum += w[i] * f(xi);
    }
    0.5 * (b - a) * sum
}

#[cfg(test)]
mod tests {
    use super::quad;

    #[test]
    fn integrates_sine_on_zero_to_pi() {
        let result = quad(|x| f64::sin(x), 0.0, std::f64::consts::PI);

        assert!((result - 2.0).abs() < 1.0e-14);
    }
}
