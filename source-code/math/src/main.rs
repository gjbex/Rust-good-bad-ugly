fn integer_arithmetic() {
    let a: i32 = 17;
    let b: i32 = 5;

    println!("Integer arithmetic:");
    println!("{a} + {b} = {}", a + b);
    println!("{a} - {b} = {}", a - b);
    println!("{a} * {b} = {}", a * b);
    println!("{a} / {b} = {}", a / b);
    println!("{a} % {b} = {}", a % b);
    println!("{a}.pow(3) = {}", i32::pow(a, 3));
    println!();
}

fn integer_division_with_negative_values() {
    let a: i32 = -17;
    let b: i32 = 5;

    println!("Integer division with negative values:");
    println!("{a} / {b} = {}", a / b);
    println!("{a} % {b} = {}", a % b);
    println!("{a}.div_euclid({b}) = {}", a.div_euclid(b));
    println!("{a}.rem_euclid({b}) = {}", a.rem_euclid(b));
    println!();
}

fn floating_point_arithmetic() {
    let x = 17.3;
    let y = 5.2;

    println!("Floating-point arithmetic:");
    println!("{x} + {y} = {}", x + y);
    println!("{x} - {y} = {}", x - y);
    println!("{x} * {y} = {}", x * y);
    println!("{x} / {y} = {}", x / y);
    println!("{x} % {y} = {}", x % y);
    println!();
}

fn mathematical_functions() {
    let angle = std::f64::consts::FRAC_PI_6;
    let value = 2.0_f64;

    println!("Mathematical functions for f64:");
    println!("sin(pi / 6) = {:.6}", angle.sin());
    println!("cos(pi / 6) = {:.6}", angle.cos());
    println!("tan(pi / 6) = {:.6}", angle.tan());
    println!("sqrt({value}) = {:.6}", value.sqrt());
    println!("{value}.powi(8) = {:.6}", value.powi(8));
    println!("{value}.powf(0.5) = {:.6}", value.powf(0.5));
    println!("exp({value}) = {:.6}", value.exp());
    println!("ln({value}) = {:.6}", value.ln());
    println!("log10({value}) = {:.6}", value.log10());
    println!();
}

fn rounding_and_absolute_values() {
    let x = -3.75_f64;

    println!("Rounding and absolute values:");
    println!("{x}.abs() = {}", x.abs());
    println!("{x}.floor() = {}", x.floor());
    println!("{x}.ceil() = {}", x.ceil());
    println!("{x}.round() = {}", x.round());
    println!("{x}.trunc() = {}", x.trunc());
}

fn main() {
    integer_arithmetic();
    integer_division_with_negative_values();
    floating_point_arithmetic();
    mathematical_functions();
    rounding_and_absolute_values();
}
