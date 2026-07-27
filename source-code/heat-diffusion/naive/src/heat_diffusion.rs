use ndarray::Array2;

pub struct System {
    grid: Array2<f64>,
    alpha: f64, // thermal diffusivity
}

impl System {
    pub fn new(grid_size: usize, alpha: f64) -> Self {
        let grid = Array2::<f64>::zeros((grid_size, grid_size));
        System { grid, alpha }
    }

    pub fn initialize_grid(
        &mut self,
        spot_temperature: f64,
        spot_radius: usize,
        boundary_temperature: f64,
    ) -> Result<(), &'static str> {
        let (rows, cols) = self.grid.dim();
        let center_row = rows / 2;
        let center_col = cols / 2;
        if spot_radius >= center_row || spot_radius >= center_col {
            return Err("Spot radius is too large for the grid size.");
        }
        for i in (center_row - spot_radius)..=(center_row + spot_radius) {
            for j in (center_col - spot_radius)..=(center_col + spot_radius) {
                let row_distance = i as f64 - center_row as f64;
                let col_distance = j as f64 - center_col as f64;
                let distance = (row_distance.powi(2) + col_distance.powi(2)).sqrt();
                if distance <= spot_radius as f64 {
                    self.grid[[i, j]] = spot_temperature;
                }
            }
        }
        self.grid[[center_row, center_col]] = spot_temperature; // central point
        for i in 0..rows {
            self.grid[[i, 0]] = boundary_temperature; // left boundary
            self.grid[[i, cols - 1]] = boundary_temperature; // right boundary
        }
        for j in 0..cols {
            self.grid[[0, j]] = boundary_temperature; // top boundary
            self.grid[[rows - 1, j]] = boundary_temperature; // bottom boundary
        }
        Ok(())
    }

    fn step(&mut self, dt: f64) -> f64 {
        let (rows, cols) = self.grid.dim();
        let mut new_grid = self.grid.clone();

        for i in 1..rows - 1 {
            for j in 1..cols - 1 {
                new_grid[[i, j]] = self.grid[[i, j]]
                    + self.alpha
                        * dt
                        * (self.grid[[i + 1, j]]
                            + self.grid[[i - 1, j]]
                            + self.grid[[i, j + 1]]
                            + self.grid[[i, j - 1]]
                            - 4.0 * self.grid[[i, j]]);
            }
        }
        let mut max_change = 0.0;
        for i in 1..rows - 1 {
            for j in 1..cols - 1 {
                let change = (new_grid[[i, j]] - self.grid[[i, j]]).abs();
                if change > max_change {
                    max_change = change;
                }
            }
        }
        self.grid = new_grid;
        max_change
    }

    pub fn run_simulation(&mut self, dt: f64, max_steps: usize, tolerance: f64) -> usize {
        for step in 1..=max_steps {
            let max_change = self.step(dt);
            if max_change < tolerance {
                return step;
            }
        }
        max_steps
    }

    pub fn get_grid(&self) -> &Array2<f64> {
        &self.grid
    }
}

impl std::fmt::Display for System {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.get_grid().rows() {
            for &value in row {
                write!(f, "{:.2} ", value)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1.0e-12;

    #[test]
    fn initializes_circular_spot_and_fixed_boundaries() {
        let grid_size = 11;
        let spot_radius = 3;
        let spot_temperature = 100.0;
        let boundary_temperature = 20.0;
        let mut system = System::new(grid_size, 0.1);

        system
            .initialize_grid(spot_temperature, spot_radius, boundary_temperature)
            .unwrap();

        let center = grid_size / 2;
        for ((row, col), &temperature) in system.grid.indexed_iter() {
            let on_boundary = row == 0 || col == 0 || row == grid_size - 1 || col == grid_size - 1;
            let row_distance = row.abs_diff(center);
            let col_distance = col.abs_diff(center);
            let inside_spot = row_distance * row_distance + col_distance * col_distance
                <= spot_radius * spot_radius;

            let expected = if on_boundary {
                boundary_temperature
            } else if inside_spot {
                spot_temperature
            } else {
                0.0
            };

            assert_eq!(
                temperature, expected,
                "unexpected temperature at ({row}, {col})"
            );
        }
    }

    #[test]
    fn rejects_spot_that_reaches_beyond_the_grid_center() {
        let mut system = System::new(10, 0.1);

        let result = system.initialize_grid(100.0, 5, 20.0);

        assert_eq!(result, Err("Spot radius is too large for the grid size."));
    }

    #[test]
    fn performs_one_hand_calculated_diffusion_step() {
        let mut system = System::new(5, 0.1);
        system.initialize_grid(100.0, 0, 20.0).unwrap();

        let max_change = system.step(0.5);

        assert!((system.grid[[2, 2]] - 80.0).abs() < EPSILON);
        assert!((max_change - 20.0).abs() < EPSILON);
        assert_eq!(system.grid[[0, 2]], 20.0);
        assert_eq!(system.grid[[4, 2]], 20.0);
        assert_eq!(system.grid[[2, 0]], 20.0);
        assert_eq!(system.grid[[2, 4]], 20.0);
    }

    #[test]
    fn reports_convergence_or_the_maximum_step_count() {
        let mut converging_system = System::new(5, 0.1);
        converging_system.initialize_grid(100.0, 0, 20.0).unwrap();

        let converged_after = converging_system.run_simulation(0.5, 10, 21.0);

        assert_eq!(converged_after, 1);

        let mut limited_system = System::new(5, 0.1);
        limited_system.initialize_grid(100.0, 0, 20.0).unwrap();

        let completed_steps = limited_system.run_simulation(0.5, 3, 0.0);

        assert_eq!(completed_steps, 3);
    }
}
