use rand::{RngExt, SeedableRng};
use rand_distr::{Normal, Uniform};

pub struct System {
    xs: Vec<f64>,
    ys: Vec<f64>,
    zs: Vec<f64>,
    vxs: Vec<f64>,
    vys: Vec<f64>,
    vzs: Vec<f64>,
    masses: Vec<f64>,
    softening_length: f64,
}

pub struct ParticleState {
    pub index: usize,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub vx: f64,
    pub vy: f64,
    pub vz: f64,
    pub mass: f64,
}

impl System {
    pub fn new(num_particles: usize, seed: u64, softening_length: f64) -> Self {
        let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
        let mut xs = Vec::with_capacity(num_particles);
        let mut ys = Vec::with_capacity(num_particles);
        let mut zs = Vec::with_capacity(num_particles);
        let mut vxs = Vec::with_capacity(num_particles);
        let mut vys = Vec::with_capacity(num_particles);
        let mut vzs = Vec::with_capacity(num_particles);
        let mut masses = Vec::with_capacity(num_particles);
        let position_distribution =
            Uniform::new(0.0, 1.0).expect("position distribution bounds should be valid");
        let velocity_distribution =
            Normal::new(0.0, 1.0).expect("velocity distribution parameters should be valid");
        let mass_distribution =
            Uniform::new(0.1, 1.0).expect("mass distribution bounds should be valid");
        for _ in 0..num_particles {
            xs.push(rng.sample(&position_distribution));
            ys.push(rng.sample(&position_distribution));
            zs.push(rng.sample(&position_distribution));
            vxs.push(rng.sample(&velocity_distribution));
            vys.push(rng.sample(&velocity_distribution));
            vzs.push(rng.sample(&velocity_distribution));
            masses.push(rng.sample(&mass_distribution));
        }
        Self {
            xs,
            ys,
            zs,
            vxs,
            vys,
            vzs,
            masses,
            softening_length,
        }
    }

    pub fn num_particles(&self) -> usize {
        self.xs.len()
    }

    fn acceleration_on(&self, index: usize) -> (f64, f64, f64) {
        let mut acceleration = (0.0, 0.0, 0.0);
        for i in 0..self.num_particles() {
            if i != index {
                let dx = self.xs[i] - self.xs[index];
                let dy = self.ys[i] - self.ys[index];
                let dz = self.zs[i] - self.zs[index];
                let distance_squared =
                    dx * dx + dy * dy + dz * dz + self.softening_length * self.softening_length;
                let distance = distance_squared.sqrt();
                let acceleration_magnitude = self.masses[i] / (distance_squared * distance);
                acceleration.0 += acceleration_magnitude * dx;
                acceleration.1 += acceleration_magnitude * dy;
                acceleration.2 += acceleration_magnitude * dz;
            }
        }
        acceleration
    }

    fn accelerations(&self) -> Vec<(f64, f64, f64)> {
        (0..self.num_particles())
            .map(|i| self.acceleration_on(i))
            .collect()
    }

    pub fn update(&mut self, dt: f64) {
        let accelerations = self.accelerations();
        let half_dt_squared = 0.5 * dt * dt;

        for i in 0..self.num_particles() {
            let (ax, ay, az) = accelerations[i];
            self.xs[i] += self.vxs[i] * dt + ax * half_dt_squared;
            self.ys[i] += self.vys[i] * dt + ay * half_dt_squared;
            self.zs[i] += self.vzs[i] * dt + az * half_dt_squared;
        }

        let new_accelerations = self.accelerations();

        for i in 0..self.num_particles() {
            let (ax, ay, az) = accelerations[i];
            let (new_ax, new_ay, new_az) = new_accelerations[i];
            self.vxs[i] += 0.5 * (ax + new_ax) * dt;
            self.vys[i] += 0.5 * (ay + new_ay) * dt;
            self.vzs[i] += 0.5 * (az + new_az) * dt;
        }
    }

    pub fn potential_energy(&self) -> f64 {
        let mut energy = 0.0;
        for i in 0..self.num_particles() {
            for j in (i + 1)..self.num_particles() {
                let dx = self.xs[i] - self.xs[j];
                let dy = self.ys[i] - self.ys[j];
                let dz = self.zs[i] - self.zs[j];
                let distance =
                    (dx * dx + dy * dy + dz * dz + self.softening_length * self.softening_length)
                        .sqrt();
                energy -= (self.masses[i] * self.masses[j]) / distance;
            }
        }
        energy
    }

    pub fn kinetic_energy(&self) -> f64 {
        let mut energy = 0.0;
        for i in 0..self.num_particles() {
            let speed_squared =
                self.vxs[i] * self.vxs[i] + self.vys[i] * self.vys[i] + self.vzs[i] * self.vzs[i];
            energy += 0.5 * self.masses[i] * speed_squared;
        }
        energy
    }

    pub fn total_energy(&self) -> f64 {
        self.kinetic_energy() + self.potential_energy()
    }

    pub fn center_of_mass(&self) -> (f64, f64, f64) {
        let mut total_mass = 0.0;
        let mut x_cm = 0.0;
        let mut y_cm = 0.0;
        let mut z_cm = 0.0;
        for i in 0..self.num_particles() {
            total_mass += self.masses[i];
            x_cm += self.masses[i] * self.xs[i];
            y_cm += self.masses[i] * self.ys[i];
            z_cm += self.masses[i] * self.zs[i];
        }
        if total_mass > 0.0 {
            x_cm /= total_mass;
            y_cm /= total_mass;
            z_cm /= total_mass;
        }
        (x_cm, y_cm, z_cm)
    }

    pub fn particle_states(&self) -> Vec<ParticleState> {
        (0..self.num_particles())
            .map(|i| ParticleState {
                index: i,
                x: self.xs[i],
                y: self.ys[i],
                z: self.zs[i],
                vx: self.vxs[i],
                vy: self.vys[i],
                vz: self.vzs[i],
                mass: self.masses[i],
            })
            .collect()
    }
}
