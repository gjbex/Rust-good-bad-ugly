fn mean_move(values: Vec<f64>) -> f64 {
    values.iter().sum::<f64>() / (values.len() as f64)
}

fn mean_borrow(values: &Vec<f64>) -> f64 {
    values.iter().sum::<f64>() / (values.len() as f64)
}

fn mean_borrow_slice(values: &[f64]) -> f64 {
    values.iter().sum::<f64>() / (values.len() as f64)
}

fn normalize_borrow_mutable_slice(v: &mut [f64]) {
    let sum: f64 = v.iter().sum();
    v.iter_mut().for_each(|x| *x /= sum);
}

fn return_vector() -> Vec<f64> {
    vec![1.0, 2.0, 3.0]
}

fn main() {
    // Scalars are copied
    let x = 5.0;
    let y = x;
    // This will work because `f64` implements the `Copy` trait.
    println!("x: {x}, y: {y}");

    // Vectors are not copied, they are moved
    let xs = vec![1.0, 2.0, 3.0];
    let ys = xs;
    // This will cause a compile error because `Vec<f64>` does not implement the `Copy` trait, and `xs` has been moved into `ys`.
    // println!("xs: {xs:?}, ys: {ys:?}");
    println!("xs can't be used here as it has been moved into ys");
    println!("ys: {ys:?}");

    // Vectors can be cloned to create a copy of the data
    let xs = vec![1.0, 2.0, 3.0];
    let ys = xs.clone();
    println!("xs: {xs:?}, ys: {ys:?}");

    // Vector remains borrowed when it shares references with another vector
    let xs = vec![1.0, 2.0, 3.0];
    let x_filtered: Vec<&f64> = xs
        .iter()
        .filter(|&&x| x > 1.5)
        .collect();
    // Modifying xs will not compile
    // xs.push(4.0); // This will cause a compile error because `xs` is borrowed by `x_filtered`.
    println!("xs: {xs:?}, x_filtered: {x_filtered:?}");

    // Copying the vector elements solves that issue
    let mut xs = vec![1.0, 2.0, 3.0];
    let x_filtered: Vec<f64> = xs
        .iter()
        .filter(|&&x| x > 1.5)
        .copied()
        .collect();
    // Modifying xs will compile
    xs.push(4.0);
    println!("xs: {xs:?}, x_filtered: {x_filtered:?}");

    // Since mean_move takes ownership of the vector, we can't use `xs` after calling it
    let xs = vec![1.0, 2.0, 3.0];
    let mean = mean_move(xs);
    println!("Mean of xs: {mean}");
    println!("xs can't be used here as it has been moved into mean_move");
    // println!("xs: {xs:?}"); // This will cause a compile error because `xs` has been moved into
    // `mean_move`.

    // To use `xs` again, we can borrow it instead of moving it
    let xs = vec![1.0, 2.0, 3.0];
    let mean = mean_borrow(&xs);
    println!("Mean of xs: {mean}");

    // We can also use a slice to borrow the vector
    let mean_slice = mean_borrow_slice(&xs);
    println!("Mean of xs: {mean_slice}");

    // To modify the vector, we can borrow it mutably
    let mut xs = vec![1.0, 2.0, 3.0];
    normalize_borrow_mutable_slice(&mut xs);
    println!("Normalized xs: {xs:?}");

    // Ownership is transferred from a function that returns a value
    let xs = return_vector();
    println!("Returned vector: {xs:?}");
}
