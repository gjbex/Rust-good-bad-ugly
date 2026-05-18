fn gcd(mut a: i32, mut b: i32) -> i32 {
    while a != b {
        if a > b {
            a -= b;
        } else {
            b -= a;
        }
    }
    a
}

fn main() {
    let a_max = 10;
    let b_max = 10;
    for a in 1..=a_max {
        for b in 1..=b_max {
            println!("gcd({a}, {b}) = {}", gcd(a, b));
        }
    }
}
