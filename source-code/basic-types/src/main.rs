fn main() {
    println!("Integers:");
    println!("{} <= i8 <= {}", i8::MIN, i8::MAX);
    println!("{} <= i16 <= {}", i16::MIN, i16::MAX);
    println!("{} <= i32 <= {}", i32::MIN, i32::MAX);
    println!("{} <= i64 <= {}", i64::MIN, i64::MAX);
    println!("{} <= i128 <= {}", i128::MIN, i128::MAX);
    println!("{} <= u8 <= {}", u8::MIN, u8::MAX);
    println!("{} <= u16 <= {}", u16::MIN, u16::MAX);
    println!("{} <= u32 <= {}", u32::MIN, u32::MAX);
    println!("{} <= u64 <= {}", u64::MIN, u64::MAX);
    println!("{} <= u128 <= {}", u128::MIN, u128::MAX);
    println!("\nFloating-point numbers:");
    println!("{:.7e} <= f32 <= {:.7e}", f32::MIN, f32::MAX);
    println!("smallest positive f32: {:.7e}", f32::MIN_POSITIVE);
    println!("epsilon for f32: {:.7e}", f32::EPSILON);
    println!("{:.15e} <= f64 <= {:.15e}", f64::MIN, f64::MAX);
    println!("smallest positive f64: {:.15e}", f64::MIN_POSITIVE);
    println!("epsilon for f64: {:.15e}", f64::EPSILON);
    println!("\nFloating-point constants:");
    println!("std::f32::consts::PI: {:.7}", std::f32::consts::PI);
    println!(
        "std::f32::consts::FRAC_1_SQRT_2: {:.7}",
        std::f32::consts::FRAC_1_SQRT_2
    );
    println!("std::f64::consts::E: {:.15}", std::f64::consts::E);
    println!("std::f64::consts::TAU: {:.15}", std::f64::consts::TAU);
    println!("\nOther types:");
    println!("char: {} <= char <= {}", char::MIN as u32, char::MAX as u32);
    println!("bool in {{{}, {}}}", false, true);
    println!("{} <= isize <= {}", isize::MIN, isize::MAX);
    println!("{} <= usize <= {}", usize::MIN, usize::MAX);
}
