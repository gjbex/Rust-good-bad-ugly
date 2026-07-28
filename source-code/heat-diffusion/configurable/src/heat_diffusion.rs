use ndarray::{Array1, Array2, ArrayView2, Axis, Zip, s};

use crate::config::InitialCondition;

const MAX_DIFFUSION_FACTOR: f64 = 0.25;

pub struct System {
    grid: Array2<f64>,
    next_grid: Array2<f64>,
    alpha: f64,
}

impl System {
    pub fn new(grid_size: usize, alpha: f64) -> Result<Self, &'static str> {
        if grid_size < 3 {
            return Err("Grid size must be at least 3.");
        }
        if !alpha.is_finite() || alpha < 0.0 {
            return Err("Thermal diffusivity must be finite and non-negative.");
        }

        let grid = Array2::<f64>::zeros((grid_size, grid_size));
        let next_grid = Array2::<f64>::zeros((grid_size, grid_size));
        Ok(Self {
            grid,
            next_grid,
            alpha,
        })
    }

    pub fn initialize_grid(
        &mut self,
        background_temperature: f64,
        boundary_temperature: f64,
        initial_condition: &InitialCondition,
    ) -> Result<(), &'static str> {
        if !background_temperature.is_finite() || !boundary_temperature.is_finite() {
            return Err("Temperatures must be finite.");
        }

        match *initial_condition {
            InitialCondition::UniformDisk {
                temperature,
                radius,
            } => self.initialize_uniform_disk(background_temperature, temperature, radius)?,
            InitialCondition::Gaussian {
                peak_temperature,
                sigma,
            } => self.initialize_gaussian(background_temperature, peak_temperature, sigma)?,
        }

        self.set_boundaries(boundary_temperature);
        self.next_grid.assign(&self.grid);
        Ok(())
    }

    fn initialize_uniform_disk(
        &mut self,
        background_temperature: f64,
        spot_temperature: f64,
        spot_radius: usize,
    ) -> Result<(), &'static str> {
        let (rows, cols) = self.grid.dim();
        let center_row = rows / 2;
        let center_col = cols / 2;
        if !spot_temperature.is_finite() {
            return Err("Temperatures must be finite.");
        }
        if spot_radius >= center_row
            || spot_radius >= center_col
            || center_row + spot_radius >= rows - 1
            || center_col + spot_radius >= cols - 1
        {
            return Err("Spot radius must fit inside the grid boundary.");
        }

        self.grid.fill(background_temperature);
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
        Ok(())
    }

    fn initialize_gaussian(
        &mut self,
        background_temperature: f64,
        peak_temperature: f64,
        sigma: f64,
    ) -> Result<(), &'static str> {
        if !peak_temperature.is_finite() {
            return Err("Temperatures must be finite.");
        }
        if !sigma.is_finite() || sigma <= 0.0 {
            return Err("Gaussian sigma must be finite and positive.");
        }

        let (rows, cols) = self.grid.dim();
        let center_row = rows / 2;
        let center_col = cols / 2;
        let row_squared =
            Array1::from_iter((0..rows).map(|row| (row as f64 - center_row as f64).powi(2)))
                .insert_axis(Axis(1));
        let col_squared =
            Array1::from_iter((0..cols).map(|col| (col as f64 - center_col as f64).powi(2)))
                .insert_axis(Axis(0));
        let radius_squared = &row_squared + &col_squared;
        let amplitude = peak_temperature - background_temperature;
        let denominator = 2.0 * sigma.powi(2);

        self.grid.assign(
            &radius_squared
                .mapv(|r2| background_temperature + amplitude * (-r2 / denominator).exp()),
        );
        Ok(())
    }

    fn set_boundaries(&mut self, boundary_temperature: f64) {
        let (rows, cols) = self.grid.dim();
        self.grid.slice_mut(s![0, ..]).fill(boundary_temperature);
        self.grid.slice_mut(s![.., 0]).fill(boundary_temperature);
        self.grid
            .slice_mut(s![rows - 1, ..])
            .fill(boundary_temperature);
        self.grid
            .slice_mut(s![.., cols - 1])
            .fill(boundary_temperature);
    }

    fn step(&mut self, dt: f64) -> f64 {
        let (rows, cols) = self.grid.dim();
        let diffusion_factor = self.alpha * dt;
        let center = self.grid.slice(s![1..rows - 1, 1..cols - 1]);
        let up = self.grid.slice(s![0..rows - 2, 1..cols - 1]);
        let down = self.grid.slice(s![2..rows, 1..cols - 1]);
        let left = self.grid.slice(s![1..rows - 1, 0..cols - 2]);
        let right = self.grid.slice(s![1..rows - 1, 2..cols]);
        let mut max_change = 0.0_f64;
        Zip::from(self.next_grid.slice_mut(s![1..rows - 1, 1..cols - 1]))
            .and(center)
            .and(up)
            .and(down)
            .and(left)
            .and(right)
            .for_each(|new, &c, &u, &d, &l, &r| {
                let new_value = c + diffusion_factor * (u + d + l + r - 4.0 * c);
                max_change = max_change.max((new_value - c).abs());
                *new = new_value;
            });
        std::mem::swap(&mut self.grid, &mut self.next_grid);
        max_change
    }

    pub fn run_simulation(
        &mut self,
        dt: f64,
        max_steps: usize,
        tolerance: f64,
    ) -> Result<usize, &'static str> {
        if !dt.is_finite() || dt <= 0.0 {
            return Err("Time step must be finite and positive.");
        }
        if !tolerance.is_finite() || tolerance <= 0.0 {
            return Err("Tolerance must be finite and positive.");
        }

        let diffusion_factor = self.alpha * dt;
        if !diffusion_factor.is_finite() || diffusion_factor > MAX_DIFFUSION_FACTOR {
            return Err("Unstable parameters: alpha * dt must not exceed 0.25.");
        }

        for step in 1..=max_steps {
            let max_change = self.step(dt);
            if max_change < tolerance {
                return Ok(step);
            }
        }
        Ok(max_steps)
    }

    pub fn get_grid(&self) -> ArrayView2<'_, f64> {
        self.grid.view()
    }
}

impl std::fmt::Display for System {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.get_grid().rows() {
            for &value in row {
                write!(f, "{value:.2} ")?;
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
    fn initializes_uniform_disk_and_fixed_boundaries() {
        let mut system = System::new(11, 0.1).unwrap();
        let initial_condition = InitialCondition::UniformDisk {
            temperature: 100.0,
            radius: 3,
        };

        system
            .initialize_grid(0.0, 20.0, &initial_condition)
            .unwrap();

        let center = 5;
        for ((row, col), &temperature) in system.get_grid().indexed_iter() {
            let on_boundary = row == 0 || col == 0 || row == 10 || col == 10;
            let row_distance = row.abs_diff(center);
            let col_distance = col.abs_diff(center);
            let inside_spot = row_distance * row_distance + col_distance * col_distance <= 9;
            let expected = if on_boundary {
                20.0
            } else if inside_spot {
                100.0
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
    fn initializes_symmetric_gaussian_with_fixed_boundaries() {
        let grid_size = 11;
        let center = grid_size / 2;
        let mut system = System::new(grid_size, 0.1).unwrap();
        let initial_condition = InitialCondition::Gaussian {
            peak_temperature: 100.0,
            sigma: 2.0,
        };

        system
            .initialize_grid(10.0, 20.0, &initial_condition)
            .unwrap();

        let grid = system.get_grid();
        assert!((grid[[center, center]] - 100.0).abs() < EPSILON);
        assert!(grid[[center, center]] > grid[[center, center + 1]]);
        assert!(grid[[center, center + 1]] > grid[[center, center + 2]]);

        for index in 0..grid_size {
            assert_eq!(grid[[0, index]], 20.0);
            assert_eq!(grid[[grid_size - 1, index]], 20.0);
            assert_eq!(grid[[index, 0]], 20.0);
            assert_eq!(grid[[index, grid_size - 1]], 20.0);
        }
        for row in 1..grid_size - 1 {
            for col in 1..grid_size - 1 {
                let value = grid[[row, col]];
                assert!((value - grid[[grid_size - 1 - row, col]]).abs() < EPSILON);
                assert!((value - grid[[row, grid_size - 1 - col]]).abs() < EPSILON);
                assert!((value - grid[[col, row]]).abs() < EPSILON);
            }
        }
    }

    #[test]
    fn uniform_disk_matches_hand_calculated_step() {
        let mut system = System::new(5, 0.1).unwrap();
        let initial_condition = InitialCondition::UniformDisk {
            temperature: 100.0,
            radius: 0,
        };
        system
            .initialize_grid(0.0, 20.0, &initial_condition)
            .unwrap();

        let max_change = system.step(0.5);

        assert!((system.grid[[2, 2]] - 80.0).abs() < EPSILON);
        assert!((max_change - 20.0).abs() < EPSILON);
    }

    #[test]
    fn rejects_invalid_initial_conditions() {
        let mut system = System::new(5, 0.1).unwrap();

        assert_eq!(
            system.initialize_grid(
                0.0,
                20.0,
                &InitialCondition::UniformDisk {
                    temperature: 100.0,
                    radius: 2,
                }
            ),
            Err("Spot radius must fit inside the grid boundary.")
        );
        assert_eq!(
            system.initialize_grid(
                0.0,
                20.0,
                &InitialCondition::Gaussian {
                    peak_temperature: 100.0,
                    sigma: 0.0,
                }
            ),
            Err("Gaussian sigma must be finite and positive.")
        );
    }

    #[test]
    fn reuses_both_grid_allocations_between_steps() {
        let mut system = System::new(5, 0.1).unwrap();
        let initial_condition = InitialCondition::UniformDisk {
            temperature: 100.0,
            radius: 0,
        };
        system
            .initialize_grid(0.0, 20.0, &initial_condition)
            .unwrap();
        let first_allocation = system.grid.as_ptr();
        let second_allocation = system.next_grid.as_ptr();

        system.step(0.5);
        assert_eq!(system.grid.as_ptr(), second_allocation);
        assert_eq!(system.next_grid.as_ptr(), first_allocation);
    }
}
