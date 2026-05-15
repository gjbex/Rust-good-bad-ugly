use std::fmt::{self, Display};
use std::ops::{Index, IndexMut};

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

    fn flat_index(&self, row: usize, col: usize) -> Option<usize> {
        if row < self.rows && col < self.cols {
            Some(row * self.cols + col)
        } else {
            None
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

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (row, col) = index;
        let flat_index = self
            .flat_index(row, col)
            .unwrap_or_else(|| panic!("matrix index ({row}, {col}) is out of bounds"));
        &self.data[flat_index]
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (row, col) = index;
        let flat_index = self
            .flat_index(row, col)
            .unwrap_or_else(|| panic!("matrix index ({row}, {col}) is out of bounds"));
        &mut self.data[flat_index]
    }
}

impl<T: Display> Display for Matrix<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..self.rows {
            for col in 0..self.cols {
                if col > 0 {
                    write!(formatter, " ")?;
                }
                write!(formatter, "{}", self[(row, col)])?;
            }
            if row + 1 < self.rows {
                writeln!(formatter)?;
            }
        }
        Ok(())
    }
}

impl<T> TryFrom<Vec<Vec<T>>> for Matrix<T> {
    type Error = String;

    fn try_from(rows: Vec<Vec<T>>) -> Result<Self, Self::Error> {
        let row_count = rows.len();
        let col_count = rows.first().map_or(0, Vec::len);

        if rows.iter().any(|row| row.len() != col_count) {
            return Err("all matrix rows must have the same length".to_string());
        }

        let data = rows.into_iter().flatten().collect();
        Ok(Self {
            rows: row_count,
            cols: col_count,
            data,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Matrix;

    #[test]
    fn indexes_matrix_elements() {
        let matrix =
            Matrix::try_from(vec![vec![1, 2], vec![3, 4]]).expect("rows have the same length");

        assert_eq!(matrix[(0, 0)], 1);
        assert_eq!(matrix[(1, 1)], 4);
    }

    #[test]
    fn mutates_matrix_elements() {
        let mut matrix = Matrix::new(2, 2, 0);
        matrix[(1, 0)] = 5;

        assert_eq!(matrix[(1, 0)], 5);
    }

    #[test]
    fn displays_matrix_rows() {
        let matrix =
            Matrix::try_from(vec![vec![1, 2], vec![3, 4]]).expect("rows have the same length");

        assert_eq!(matrix.to_string(), "1 2\n3 4");
    }

    #[test]
    fn rejects_ragged_rows() {
        let result = Matrix::try_from(vec![vec![1, 2], vec![3]]);

        assert!(result.is_err());
    }
}
