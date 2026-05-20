#include "system.h"

#include <cmath>
#include <random>

System::System(std::size_t num_particles, std::uint64_t seed, double softening_length)
    : softening_length_{softening_length}
{
    xs_.reserve(num_particles);
    ys_.reserve(num_particles);
    zs_.reserve(num_particles);
    vxs_.reserve(num_particles);
    vys_.reserve(num_particles);
    vzs_.reserve(num_particles);
    masses_.reserve(num_particles);

    std::mt19937_64 rng(seed);
    std::uniform_real_distribution<double> position_distribution(0.0, 1.0);
    std::normal_distribution<double> velocity_distribution(0.0, 1.0);
    std::uniform_real_distribution<double> mass_distribution(0.1, 1.0);

    for (std::size_t i = 0; i < num_particles; ++i) {
        xs_.push_back(position_distribution(rng));
        ys_.push_back(position_distribution(rng));
        zs_.push_back(position_distribution(rng));
        vxs_.push_back(velocity_distribution(rng));
        vys_.push_back(velocity_distribution(rng));
        vzs_.push_back(velocity_distribution(rng));
        masses_.push_back(mass_distribution(rng));
    }
}

std::size_t System::num_particles() const
{
    return xs_.size();
}

std::tuple<double, double, double> System::acceleration_on(std::size_t index) const
{
    double ax = 0.0;
    double ay = 0.0;
    double az = 0.0;
    for (std::size_t i = 0; i < num_particles(); ++i) {
        if (i != index) {
            const double dx = xs_[i] - xs_[index];
            const double dy = ys_[i] - ys_[index];
            const double dz = zs_[i] - zs_[index];
            const double distance_squared =
                dx * dx + dy * dy + dz * dz + softening_length_ * softening_length_;
            const double distance = std::sqrt(distance_squared);
            const double acceleration_magnitude = masses_[i] / (distance_squared * distance);
            ax += acceleration_magnitude * dx;
            ay += acceleration_magnitude * dy;
            az += acceleration_magnitude * dz;
        }
    }
    return {ax, ay, az};
}

std::vector<std::tuple<double, double, double>> System::accelerations() const
{
    std::vector<std::tuple<double, double, double>> values;
    values.reserve(num_particles());
    for (std::size_t i = 0; i < num_particles(); ++i) {
        values.push_back(acceleration_on(i));
    }
    return values;
}

void System::update(double dt)
{
    const auto old_accelerations = accelerations();
    const double half_dt_squared = 0.5 * dt * dt;

    for (std::size_t i = 0; i < num_particles(); ++i) {
        const auto [ax, ay, az] = old_accelerations[i];
        xs_[i] += vxs_[i] * dt + ax * half_dt_squared;
        ys_[i] += vys_[i] * dt + ay * half_dt_squared;
        zs_[i] += vzs_[i] * dt + az * half_dt_squared;
    }

    const auto new_accelerations = accelerations();

    for (std::size_t i = 0; i < num_particles(); ++i) {
        const auto [ax, ay, az] = old_accelerations[i];
        const auto [new_ax, new_ay, new_az] = new_accelerations[i];
        vxs_[i] += 0.5 * (ax + new_ax) * dt;
        vys_[i] += 0.5 * (ay + new_ay) * dt;
        vzs_[i] += 0.5 * (az + new_az) * dt;
    }
}

double System::potential_energy() const
{
    double energy = 0.0;
    for (std::size_t i = 0; i < num_particles(); ++i) {
        for (std::size_t j = i + 1; j < num_particles(); ++j) {
            const double dx = xs_[i] - xs_[j];
            const double dy = ys_[i] - ys_[j];
            const double dz = zs_[i] - zs_[j];
            const double distance =
                std::sqrt(dx * dx + dy * dy + dz * dz + softening_length_ * softening_length_);
            energy -= (masses_[i] * masses_[j]) / distance;
        }
    }
    return energy;
}

double System::kinetic_energy() const
{
    double energy = 0.0;
    for (std::size_t i = 0; i < num_particles(); ++i) {
        const double speed_squared =
            vxs_[i] * vxs_[i] + vys_[i] * vys_[i] + vzs_[i] * vzs_[i];
        energy += 0.5 * masses_[i] * speed_squared;
    }
    return energy;
}

double System::total_energy() const
{
    return kinetic_energy() + potential_energy();
}

std::tuple<double, double, double> System::center_of_mass() const
{
    double total_mass = 0.0;
    double x_cm = 0.0;
    double y_cm = 0.0;
    double z_cm = 0.0;
    for (std::size_t i = 0; i < num_particles(); ++i) {
        total_mass += masses_[i];
        x_cm += masses_[i] * xs_[i];
        y_cm += masses_[i] * ys_[i];
        z_cm += masses_[i] * zs_[i];
    }
    if (total_mass > 0.0) {
        x_cm /= total_mass;
        y_cm /= total_mass;
        z_cm /= total_mass;
    }
    return {x_cm, y_cm, z_cm};
}

std::vector<ParticleState> System::particle_states() const
{
    std::vector<ParticleState> states;
    states.reserve(num_particles());
    for (std::size_t i = 0; i < num_particles(); ++i) {
        states.push_back(ParticleState{
            i,
            xs_[i],
            ys_[i],
            zs_[i],
            vxs_[i],
            vys_[i],
            vzs_[i],
            masses_[i],
        });
    }
    return states;
}
