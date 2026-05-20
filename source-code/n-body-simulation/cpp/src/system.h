#ifndef N_BODY_SIMULATION_CPP_SYSTEM_H
#define N_BODY_SIMULATION_CPP_SYSTEM_H

#include <cstddef>
#include <cstdint>
#include <tuple>
#include <vector>

struct ParticleState {
    std::size_t index;
    double x;
    double y;
    double z;
    double vx;
    double vy;
    double vz;
    double mass;
};

class System {
public:
    System(std::size_t num_particles, std::uint64_t seed, double softening_length);

    [[nodiscard]] std::size_t num_particles() const;
    void update(double dt);

    [[nodiscard]] double potential_energy() const;
    [[nodiscard]] double kinetic_energy() const;
    [[nodiscard]] double total_energy() const;
    [[nodiscard]] std::tuple<double, double, double> center_of_mass() const;
    [[nodiscard]] std::vector<ParticleState> particle_states() const;

private:
    [[nodiscard]] std::tuple<double, double, double> acceleration_on(std::size_t index) const;
    [[nodiscard]] std::vector<std::tuple<double, double, double>> accelerations() const;

    std::vector<double> xs_;
    std::vector<double> ys_;
    std::vector<double> zs_;
    std::vector<double> vxs_;
    std::vector<double> vys_;
    std::vector<double> vzs_;
    std::vector<double> masses_;
    double softening_length_;
};

#endif
