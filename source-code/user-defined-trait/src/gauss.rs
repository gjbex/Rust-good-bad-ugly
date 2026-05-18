use crate::quadrature::QuadratureRule;

pub struct GaussLegendre10;

impl QuadratureRule for GaussLegendre10 {
    fn integrate(&self, f: &dyn Fn(f64) -> f64, a: f64, b: f64) -> f64 {
        let weights = [
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
        let points = [
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

        let mut sum = 0.0;
        for i in 0..weights.len() {
            let xi = 0.5 * (b - a) * points[i] + 0.5 * (a + b);
            sum += weights[i] * f(xi);
        }
        0.5 * (b - a) * sum
    }

    fn name(&self) -> &'static str {
        "10-point Gauss-Legendre"
    }
}

#[cfg(test)]
mod tests {
    use super::GaussLegendre10;
    use crate::quadrature::QuadratureRule;

    #[test]
    fn integrates_sine_on_zero_to_pi() {
        let rule = GaussLegendre10;
        let result = rule.integrate(&|x| f64::sin(x), 0.0, std::f64::consts::PI);

        assert!((result - 2.0).abs() < 1.0e-14);
    }
}
