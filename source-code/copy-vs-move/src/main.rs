fn mean_wrong(values: Vec<f64>) -> f64 {
    values.iter().sum::<f64>() / (values.len() as f64)
}

fn mean_right(values: &[f64]) -> f64 {
    values.iter().sum::<f64>() / (values.len() as f64)
}

fn main() {
    let data1 = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    let mean = mean_wrong(data1);
    println!("Mean (wrong) of data 1: {}", mean);
    println!("Data1 can't be used here");
    // The following line will cause a compile error because `data` has been moved into
    // `mean_wrong` and is no longer available in `main`.
    // println!("Data {data1:?}"); // This will cause a compile error because `data` has been moved
    let data2 = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
    let mean = mean_right(&data2);
    println!("Mean (right) of data 2: {}", mean);
    // The following line will work because `data2` is still available in `main`                    
    // after being borrowed by `mean_right`.
    println!("Data2 {data2:?}");
}
