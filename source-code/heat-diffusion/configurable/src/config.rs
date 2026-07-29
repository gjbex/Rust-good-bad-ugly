use std::{error::Error, fs, io, path::PathBuf};

use serde::Deserialize;

const MAX_DIFFUSION_FACTOR: f64 = 0.25;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub grid: GridConfig,
    pub material: MaterialConfig,
    pub solver: SolverConfig,
    pub initial_condition: InitialCondition,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GridConfig {
    pub size: usize,
    pub background_temperature: f64,
    pub boundary_temperature: f64,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MaterialConfig {
    pub thermal_diffusivity: f64,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SolverConfig {
    pub time_step: f64,
    pub max_steps: usize,
    pub tolerance: f64,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case", deny_unknown_fields)]
pub enum InitialCondition {
    UniformDisk { temperature: f64, radius: usize },
    Gaussian { peak_temperature: f64, sigma: f64 },
}

impl Config {
    pub fn load(path: PathBuf) -> Result<Self, Box<dyn Error>> {
        let text = fs::read_to_string(path)?;
        let config: Self = toml::from_str(&text)?;
        config
            .validate()
            .map_err(|message| io::Error::new(io::ErrorKind::InvalidData, message))?;
        Ok(config)
    }

    fn validate(&self) -> Result<(), &'static str> {
        if self.grid.size < 3 {
            return Err("Grid size must be at least 3.");
        }
        if !self.grid.background_temperature.is_finite()
            || !self.grid.boundary_temperature.is_finite()
        {
            return Err("Temperatures must be finite.");
        }
        if !self.material.thermal_diffusivity.is_finite() || self.material.thermal_diffusivity < 0.0
        {
            return Err("Thermal diffusivity must be finite and non-negative.");
        }
        if !self.solver.time_step.is_finite() || self.solver.time_step <= 0.0 {
            return Err("Time step must be finite and positive.");
        }
        if !self.solver.tolerance.is_finite() || self.solver.tolerance <= 0.0 {
            return Err("Tolerance must be finite and positive.");
        }

        let diffusion_factor = self.material.thermal_diffusivity * self.solver.time_step;
        if !diffusion_factor.is_finite() || diffusion_factor > MAX_DIFFUSION_FACTOR {
            return Err("Unstable parameters: alpha * dt must not exceed 0.25.");
        }

        match self.initial_condition {
            InitialCondition::UniformDisk {
                temperature,
                radius,
            } => {
                if !temperature.is_finite() {
                    return Err("Temperatures must be finite.");
                }
                let center = self.grid.size / 2;
                if radius >= center || center + radius >= self.grid.size - 1 {
                    return Err("Spot radius must fit inside the grid boundary.");
                }
            }
            InitialCondition::Gaussian {
                peak_temperature,
                sigma,
            } => {
                if !peak_temperature.is_finite() {
                    return Err("Temperatures must be finite.");
                }
                if !sigma.is_finite() || sigma <= 0.0 {
                    return Err("Gaussian sigma must be finite and positive.");
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    const UNIFORM_CONFIG: &str = include_str!("../configs/uniform-spot.toml");
    const GAUSSIAN_CONFIG: &str = include_str!("../configs/gaussian-spot.toml");

    #[test]
    fn parses_uniform_disk_configuration() {
        let config: Config = toml::from_str(UNIFORM_CONFIG).unwrap();

        assert_eq!(config.grid.size, 21);
        assert!(matches!(
            config.initial_condition,
            InitialCondition::UniformDisk {
                temperature: 100.0,
                radius: 5
            }
        ));
        assert_eq!(config.validate(), Ok(()));
    }

    #[test]
    fn parses_gaussian_configuration() {
        let config: Config = toml::from_str(GAUSSIAN_CONFIG).unwrap();

        assert!(matches!(
            config.initial_condition,
            InitialCondition::Gaussian {
                peak_temperature: 100.0,
                sigma: 3.0
            }
        ));
        assert_eq!(config.validate(), Ok(()));
    }

    #[test]
    fn rejects_unknown_configuration_fields() {
        let text = UNIFORM_CONFIG.replace("radius = 5", "radius = 5\nraduis = 5");

        let error = toml::from_str::<Config>(&text).unwrap_err();

        assert!(error.to_string().contains("unknown field"));
    }

    #[test]
    fn rejects_invalid_initial_condition_parameters() {
        let invalid_radius = UNIFORM_CONFIG.replace("radius = 5", "radius = 11");
        let config: Config = toml::from_str(&invalid_radius).unwrap();
        assert_eq!(
            config.validate(),
            Err("Spot radius must fit inside the grid boundary.")
        );

        let invalid_sigma = GAUSSIAN_CONFIG.replace("sigma = 3.0", "sigma = 0.0");
        let config: Config = toml::from_str(&invalid_sigma).unwrap();
        assert_eq!(
            config.validate(),
            Err("Gaussian sigma must be finite and positive.")
        );
    }

    #[test]
    fn rejects_unstable_solver_parameters() {
        let text = UNIFORM_CONFIG.replace("time_step = 0.01", "time_step = 3.0");
        let config: Config = toml::from_str(&text).unwrap();

        assert_eq!(
            config.validate(),
            Err("Unstable parameters: alpha * dt must not exceed 0.25.")
        );
    }

    #[test]
    fn reports_missing_configuration_files() {
        let result = Config::load(Path::new("missing-config.toml").to_path_buf());

        assert!(result.is_err());
    }
}
