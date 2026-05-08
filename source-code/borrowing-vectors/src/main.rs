fn mean(data: &[f64]) -> f64 {
    let sum: f64 = data.iter().sum();
    sum / (data.len() as f64)
}

fn normalize(data: &mut [f64]) {
    let mean_value = mean(data);
    for value in data.iter_mut() {
        *value /= mean_value;
    }
}

fn main() {
    let mut data = vec![3.1, 2.4, 5.6, 1.2, 4.8];
    println!("Original data: {:?}", data);

    {
        let first_value = &data[0];
        let this_mean = mean(&data);
        println!("First value: {first_value}, no problem using data");
    }

    // Uncommenting these lines breaks Rust's borrowing rules:
    //
    // {
    //     let first_value = &data[0];
    //     normalize(&mut data);
    //     println!("First value before normalization: {first_value}");
    // }
    //
    // `first_value` is a shared borrow into `data`.  Since that borrow is used
    // after the call to `normalize`, Rust will not allow `data` to be mutably
    // borrowed by `normalize(&mut data)` in between.

    let mean_value = mean(&data);
    println!("Mean: {:.3}", mean_value);
    normalize(&mut data);
    println!("Normalized data: {:?}", data);
    let new_mean = mean(&data);
    println!("New Mean after normalization: {:.3}", new_mean);
}
