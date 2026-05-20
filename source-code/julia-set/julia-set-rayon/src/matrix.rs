pub struct Matrix<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>,
}

impl<T> Matrix<T> {
    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    fn index(&self, row: usize, col: usize) -> Option<usize> {
        if row < self.rows && col < self.cols {
            Some(row * self.cols + col)
        } else {
            None
        }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        self.index(row, col).map(|index| &self.data[index])
    }

    pub fn set(&mut self, row: usize, col: usize, value: T) -> Result<(), String> {
        let index = self
            .index(row, col)
            .ok_or_else(|| format!("matrix index ({row}, {col}) is out of bounds"))?;
        self.data[index] = value;
        Ok(())
    }

    pub fn from_vec(rows: usize, cols: usize, data: Vec<T>) -> Result<Self, String> {
        if data.len() == rows * cols {
            Ok(Self { rows, cols, data })
        } else {
            Err(format!(
                "matrix data has {} elements, but shape ({rows}, {cols}) requires {}",
                data.len(),
                rows * cols
            ))
        }
    }
}

impl<T: Clone> Matrix<T> {
    pub fn new(rows: usize, cols: usize, value: T) -> Self {
        Self {
            rows,
            cols,
            data: vec![value; rows * cols],
        }
    }
}
