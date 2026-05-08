use std::any::type_name_of_val;

fn compute_polynom(x: f32) -> f32 {
    let a = 3.0;
    let b = 2.0;
    let c = 1.0;
    println!("Type of a: {}", type_name_of_val(&a));
    a * x * x + b * x + c
}

fn main() {
    let x = 5.0;
    println!("Type of x: {}", type_name_of_val(&x));
    let result = compute_polynom(x);
    println!("The result of the polynomial for x = {:.1} is {:.1}", x, result);
}
