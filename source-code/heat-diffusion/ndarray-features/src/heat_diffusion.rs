use ndarray::{Array2, ArrayView2, Zip, s};

const MAX_DIFFUSION_FACTOR: f64 = 0.25;

pub struct System {
    grid: Array2<f64>,
    next_grid: Array2<f64>,
    alpha: f64, // thermal diffusivity
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
        Ok(System {
            grid,
            next_grid,
            alpha,
        })
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
        if !spot_temperature.is_finite() || !boundary_temperature.is_finite() {
            return Err("Temperatures must be finite.");
        }
        if spot_radius >= center_row
            || spot_radius >= center_col
            || center_row + spot_radius >= rows - 1
            || center_col + spot_radius >= cols - 1
        {
            return Err("Spot radius must fit inside the grid boundary.");
        }
        self.grid.fill(0.0);

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
        self.next_grid.assign(&self.grid);
        Ok(())
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
        let mut system = System::new(grid_size, 0.1).unwrap();

        system
            .initialize_grid(spot_temperature, spot_radius, boundary_temperature)
            .unwrap();

        let center = grid_size / 2;
        for ((row, col), &temperature) in system.get_grid().indexed_iter() {
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
        let mut system = System::new(10, 0.1).unwrap();

        let result = system.initialize_grid(100.0, 5, 20.0);

        assert_eq!(
            result,
            Err("Spot radius must fit inside the grid boundary.")
        );
    }

    #[test]
    fn performs_one_hand_calculated_diffusion_step() {
        let mut system = System::new(5, 0.1).unwrap();
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
        let mut converging_system = System::new(5, 0.1).unwrap();
        converging_system.initialize_grid(100.0, 0, 20.0).unwrap();

        let converged_after = converging_system.run_simulation(0.5, 10, 21.0).unwrap();

        assert_eq!(converged_after, 1);

        let mut limited_system = System::new(5, 0.1).unwrap();
        limited_system.initialize_grid(100.0, 0, 20.0).unwrap();

        let completed_steps = limited_system.run_simulation(0.5, 3, 1.0e-12).unwrap();

        assert_eq!(completed_steps, 3);
    }

    #[test]
    fn rejects_invalid_construction_parameters() {
        assert_eq!(
            System::new(2, 0.1).err(),
            Some("Grid size must be at least 3.")
        );
        assert_eq!(
            System::new(5, -0.1).err(),
            Some("Thermal diffusivity must be finite and non-negative.")
        );
        assert_eq!(
            System::new(5, f64::NAN).err(),
            Some("Thermal diffusivity must be finite and non-negative.")
        );
    }

    #[test]
    fn rejects_invalid_or_unstable_simulation_parameters() {
        let mut system = System::new(5, 0.1).unwrap();
        system.initialize_grid(100.0, 0, 20.0).unwrap();

        assert_eq!(
            system.run_simulation(0.0, 10, 1.0e-3),
            Err("Time step must be finite and positive.")
        );
        assert_eq!(
            system.run_simulation(0.5, 10, 0.0),
            Err("Tolerance must be finite and positive.")
        );
        assert_eq!(
            system.run_simulation(3.0, 10, 1.0e-3),
            Err("Unstable parameters: alpha * dt must not exceed 0.25.")
        );
    }

    #[test]
    fn rejects_non_finite_temperatures() {
        let mut system = System::new(5, 0.1).unwrap();

        assert_eq!(
            system.initialize_grid(f64::NAN, 0, 20.0),
            Err("Temperatures must be finite.")
        );
        assert_eq!(
            system.initialize_grid(100.0, 0, f64::INFINITY),
            Err("Temperatures must be finite.")
        );
    }

    #[test]
    fn reuses_both_grid_allocations_between_steps() {
        let mut system = System::new(5, 0.1).unwrap();
        system.initialize_grid(100.0, 0, 20.0).unwrap();
        assert_eq!(system.grid, system.next_grid);

        let first_allocation = system.grid.as_ptr();
        let second_allocation = system.next_grid.as_ptr();

        system.step(0.5);
        assert_eq!(system.grid.as_ptr(), second_allocation);
        assert_eq!(system.next_grid.as_ptr(), first_allocation);

        system.step(0.5);
        assert_eq!(system.grid.as_ptr(), first_allocation);
        assert_eq!(system.next_grid.as_ptr(), second_allocation);
    }
}
