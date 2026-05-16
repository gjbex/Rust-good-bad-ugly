pub trait QuadratureRule {
    fn integrate(&self, f: &dyn Fn(f64) -> f64, a: f64, b: f64) -> f64;

    fn name(&self) -> &'static str;
}
