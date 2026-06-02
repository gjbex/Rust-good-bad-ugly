use num_traits::{Float, FromPrimitive, ToPrimitive};

struct Stats<T: Float + FromPrimitive> {
    sum: T,
    sum_sqr: T,
    count: usize,
}

impl<T: Float + FromPrimitive> Stats<T> {
    fn new() -> Self {
        Stats {
            sum: T::zero(),
            sum_sqr: T::zero(),
            count: 0,
        }
    }

    fn add<U>(&mut self, value: U) -> Option<()>
    where
        U: ToPrimitive,
    {
        let value = T::from(value)?;
        self.sum = self.sum + value;
        self.sum_sqr = self.sum_sqr + value * value;
        self.count += 1;
        Some(())
    }

    fn count(&self) -> usize {
        self.count
    }

    fn mean(&self) -> Option<T> {
        if self.count == 0 {
            None
        } else {
            let count = T::from(self.count)?;
            Some(self.sum / count)
        }
    }

    fn stddev(&self) -> Option<T> {
        if self.count < 2 {
            None
        } else {
            let count = T::from(self.count)?;
            let mean = self.sum / count;
            let variance = (self.sum_sqr / count) - (mean * mean);
            Some(variance.sqrt())
        }
    }
}

fn main() {
    let mut stats = Stats::<f64>::new();
    stats.add(1).unwrap();
    stats.add(2 as i16).unwrap();
    stats.add(3.3).unwrap();
    match (stats.count(), stats.mean(), stats.stddev()) {
        (count, Some(mean), Some(stddev)) => println!("Count: {count}, Mean: {mean}, Stddev: {stddev}"),
        _ => println!("Error: no result"),
    }
}
