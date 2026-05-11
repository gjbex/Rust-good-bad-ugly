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
