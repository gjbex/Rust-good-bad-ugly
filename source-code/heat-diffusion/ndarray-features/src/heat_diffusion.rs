use ndarray::{Array2, Zip, s};

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
        // initialize the central splot
        let mut spot = self.grid.slice_mut(s![
            center_row - spot_radius..center_row + spot_radius + 1,
            center_col - spot_radius..center_col + spot_radius + 1
        ]);
        for ((row, col), temperature) in spot.indexed_iter_mut() {
            let row_distance = row.abs_diff(spot_radius);
            let col_distance = col.abs_diff(spot_radius);

            if row_distance.pow(2) + col_distance.pow(2) <= spot_radius.pow(2) {
                *temperature = spot_temperature;
            }
        }
        // initialize the boundary conditions
        self.grid.slice_mut(s![0, ..]).fill(boundary_temperature);
        self.grid.slice_mut(s![.., 0]).fill(boundary_temperature);
        self.grid
            .slice_mut(s![rows - 1, ..])
            .fill(boundary_temperature);
        self.grid
            .slice_mut(s![.., cols - 1])
            .fill(boundary_temperature);
        Ok(())
    }

    fn step(&mut self, dt: f64) -> f64 {
        let (rows, cols) = self.grid.dim();
        let center = self.grid.slice(s![1..rows - 1, 1..cols - 1]);
        let up = self.grid.slice(s![0..rows - 2, 1..cols - 1]);
        let down = self.grid.slice(s![2..rows, 1..cols - 1]);
        let left = self.grid.slice(s![1..rows - 1, 0..cols - 2]);
        let right = self.grid.slice(s![1..rows - 1, 2..cols]);
        let mut new_grid = self.grid.clone();
        Zip::from(new_grid.slice_mut(s![1..rows - 1, 1..cols - 1]))
            .and(center)
            .and(up)
            .and(down)
            .and(left)
            .and(right)
            .for_each(|new, &c, &u, &d, &l, &r| {
                *new = c + self.alpha * dt * (u + d + l + r - 4.0 * c);
            });
        let max_change = new_grid
            .iter()
            .zip(self.grid.iter())
            .map(|(new, old)| (new - old).abs())
            .fold(0.0_f64, |a, b| a.max(b));
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
}

impl std::fmt::Display for System {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.grid.rows() {
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
