fn rhs(x: f64, dxdt: &mut f64, _t: f64) {
    *dxdt = -x;
}

fn main() {
    let mut x = 1.0;
    let mut dxdt = 0.0;
    let mut t = 0.0;
    let delta_t = 0.1;
    println!("{:.5} {:.5}", t, x);
    for _ in 0..20 {
        t += delta_t;
        rhs(x, &mut dxdt, t);
        x += dxdt * delta_t;
        println!("{:.5} {:.5}", t, x);
    }
}
