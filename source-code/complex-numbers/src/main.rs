use num_complex::Complex64;

fn main() {
    let z1 = Complex64 { re: 1.0, im: 2.0 };
    println!("z1 = {z1}");
    let z2 = Complex64 { re: 3.0, im: 4.0 };
    println!("z2 = {z2}");
    let z3 = z1 + z2;
    println!("z1 + z2 = {z3}");
    println!("z1 * z2 = {}", z1 * z2);

    println!("re(z1) = {}", z1.re);
    println!("im(z1) = {}", z1.im);
    println!("|z1| = {}", z1.norm());
}
