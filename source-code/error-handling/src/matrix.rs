pub struct Matrix {
    rows: usize,
    cols: usize,
    data: Vec<f64>,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            data: vec![0.0; rows * cols],
        }
    }

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

    pub fn get(&self, row: usize, col: usize) -> Option<f64> {
        self.index(row, col).map(|index| self.data[index])
    }

    pub fn set(&mut self, row: usize, col: usize, value: f64) -> Result<(), String> {
        let index = self
            .index(row, col)
            .ok_or_else(|| format!("matrix index ({row}, {col}) is out of bounds"))?;
        self.data[index] = value;
        Ok(())
    }
}
