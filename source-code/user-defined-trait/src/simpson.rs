use crate::quadrature::QuadratureRule;

pub struct Simpson {
    subdivisions: usize,
}

impl Simpson {
    pub fn new(subdivisions: usize) -> Self {
        assert!(subdivisions > 0, "number of subdivisions must be positive");
        assert!(
            subdivisions % 2 == 0,
            "Simpson's rule requires an even number of subdivisions"
        );
        Self { subdivisions }
    }
}

impl QuadratureRule for Simpson {
    fn integrate(&self, f: &dyn Fn(f64) -> f64, a: f64, b: f64) -> f64 {
        let h = (b - a) / self.subdivisions as f64;
        let mut sum = f(a) + f(b);

        for i in 1..self.subdivisions {
            let x = a + i as f64 * h;
            let weight = if i % 2 == 0 { 2.0 } else { 4.0 };
            sum += weight * f(x);
        }

        h * sum / 3.0
    }

    fn name(&self) -> &'static str {
        "composite Simpson"
    }
}

#[cfg(test)]
mod tests {
    use super::Simpson;
    use crate::quadrature::QuadratureRule;

    #[test]
    fn integrates_sine_on_zero_to_pi() {
        let rule = Simpson::new(1000);
        let result = rule.integrate(&|x| f64::sin(x), 0.0, std::f64::consts::PI);

        assert!((result - 2.0).abs() < 2.0e-12);
    }
}
