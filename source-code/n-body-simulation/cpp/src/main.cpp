#include "system.h"

#include <cstddef>
#include <cstdint>
#include <cstdlib>
#include <exception>
#include <fstream>
#include <iomanip>
#include <iostream>
#include <limits>
#include <stdexcept>
#include <string>

struct Args {
    std::size_t num_particles = 100;
    std::uint64_t seed = 1234;
    double delta_time = 0.001;
    std::size_t num_steps = 100;
    double softening = 0.01;
    std::string save_evolution;
    std::string save_states;
};

void print_usage(const char* program)
{
    std::cout
        << "Usage: " << program << " [OPTIONS]\n\n"
        << "Options:\n"
        << "  --num-particles <N>       Number of particles in the simulation [default: 100]\n"
        << "  --seed <N>                Random seed for initializing the system [default: 1234]\n"
        << "  --delta-time <VALUE>      Delta time for the simulation steps [default: 0.001]\n"
        << "  --steps <N>               Number of simulation steps to run [default: 100]\n"
        << "  --softening <VALUE>       Gravitational softening length [default: 0.01]\n"
        << "  --save-evolution <FILE>   Save energy and center of mass evolution as CSV\n"
        << "  --save-states <FILE>      Save particle states as CSV\n"
        << "  -h, --help                Print help\n";
}

std::string option_value(int& index, int argc, char* argv[], const std::string& option)
{
    const std::string argument = argv[index];
    const std::string prefix = option + "=";
    if (argument.rfind(prefix, 0) == 0) {
        return argument.substr(prefix.size());
    }
    if (index + 1 >= argc) {
        throw std::invalid_argument("missing value for " + option);
    }
    ++index;
    return argv[index];
}

void require_digits(const std::string& value, const std::string& option)
{
    if (value.empty()) {
        throw std::invalid_argument("invalid integer for " + option + ": " + value);
    }
    for (const unsigned char character : value) {
        if (character < '0' || character > '9') {
            throw std::invalid_argument("invalid integer for " + option + ": " + value);
        }
    }
}

std::size_t parse_size(const std::string& value, const std::string& option)
{
    require_digits(value, option);
    std::size_t parsed_chars = 0;
    unsigned long long parsed = 0;
    try {
        parsed = std::stoull(value, &parsed_chars);
    } catch (const std::exception&) {
        throw std::invalid_argument("invalid integer for " + option + ": " + value);
    }
    if (parsed_chars != value.size() || parsed > std::numeric_limits<std::size_t>::max()) {
        throw std::invalid_argument("invalid integer for " + option + ": " + value);
    }
    return static_cast<std::size_t>(parsed);
}

std::uint64_t parse_u64(const std::string& value, const std::string& option)
{
    require_digits(value, option);
    std::size_t parsed_chars = 0;
    unsigned long long parsed = 0;
    try {
        parsed = std::stoull(value, &parsed_chars);
    } catch (const std::exception&) {
        throw std::invalid_argument("invalid integer for " + option + ": " + value);
    }
    if (parsed_chars != value.size()
        || parsed > static_cast<unsigned long long>(std::numeric_limits<std::uint64_t>::max())) {
        throw std::invalid_argument("invalid integer for " + option + ": " + value);
    }
    return static_cast<std::uint64_t>(parsed);
}

double parse_double(const std::string& value, const std::string& option)
{
    std::size_t parsed_chars = 0;
    double parsed = 0.0;
    try {
        parsed = std::stod(value, &parsed_chars);
    } catch (const std::exception&) {
        throw std::invalid_argument("invalid floating-point value for " + option + ": " + value);
    }
    if (parsed_chars != value.size()) {
        throw std::invalid_argument("invalid floating-point value for " + option + ": " + value);
    }
    return parsed;
}

Args parse_args(int argc, char* argv[])
{
    Args args;
    for (int index = 1; index < argc; ++index) {
        const std::string argument = argv[index];
        if (argument == "-h" || argument == "--help") {
            print_usage(argv[0]);
            std::exit(0);
        } else if (argument == "--num-particles" || argument.rfind("--num-particles=", 0) == 0) {
            args.num_particles = parse_size(option_value(index, argc, argv, "--num-particles"),
                "--num-particles");
        } else if (argument == "--seed" || argument.rfind("--seed=", 0) == 0) {
            args.seed = parse_u64(option_value(index, argc, argv, "--seed"), "--seed");
        } else if (argument == "--delta-time" || argument.rfind("--delta-time=", 0) == 0) {
            args.delta_time =
                parse_double(option_value(index, argc, argv, "--delta-time"), "--delta-time");
        } else if (argument == "--steps" || argument.rfind("--steps=", 0) == 0) {
            args.num_steps = parse_size(option_value(index, argc, argv, "--steps"), "--steps");
        } else if (argument == "--softening" || argument.rfind("--softening=", 0) == 0) {
            args.softening =
                parse_double(option_value(index, argc, argv, "--softening"), "--softening");
        } else if (argument == "--save-evolution" || argument.rfind("--save-evolution=", 0) == 0) {
            args.save_evolution = option_value(index, argc, argv, "--save-evolution");
        } else if (argument == "--save-states" || argument.rfind("--save-states=", 0) == 0) {
            args.save_states = option_value(index, argc, argv, "--save-states");
        } else {
            throw std::invalid_argument("unknown option: " + argument);
        }
    }
    return args;
}

std::ofstream open_output_file(const std::string& filename)
{
    std::ofstream file(filename);
    if (!file) {
        throw std::runtime_error("failed to create output file: " + filename);
    }
    file << std::setprecision(17);
    return file;
}

void write_evolution_header(std::ofstream& writer)
{
    writer << "step,potential_energy,kinetic_energy,total_energy,center_of_mass_x,"
              "center_of_mass_y,center_of_mass_z\n";
}

void write_states_header(std::ofstream& writer)
{
    writer << "step,particle,x,y,z,vx,vy,vz,mass\n";
}

void write_evolution_record(std::ofstream* writer, std::size_t step, const System& system)
{
    if (writer == nullptr) {
        return;
    }
    const auto [center_of_mass_x, center_of_mass_y, center_of_mass_z] = system.center_of_mass();
    const double potential_energy = system.potential_energy();
    const double kinetic_energy = system.kinetic_energy();
    *writer << step << ',' << potential_energy << ',' << kinetic_energy << ','
            << potential_energy + kinetic_energy << ',' << center_of_mass_x << ','
            << center_of_mass_y << ',' << center_of_mass_z << '\n';
}

void write_state_records(std::ofstream* writer, std::size_t step, const System& system)
{
    if (writer == nullptr) {
        return;
    }
    for (const auto& state : system.particle_states()) {
        *writer << step << ',' << state.index << ',' << state.x << ',' << state.y << ','
                << state.z << ',' << state.vx << ',' << state.vy << ',' << state.vz << ','
                << state.mass << '\n';
    }
}

int main(int argc, char* argv[])
{
    try {
        const Args args = parse_args(argc, argv);
        System system(args.num_particles, args.seed, args.softening);

        std::ofstream evolution_file;
        std::ofstream states_file;
        std::ofstream* evolution_writer = nullptr;
        std::ofstream* states_writer = nullptr;

        if (!args.save_evolution.empty()) {
            evolution_file = open_output_file(args.save_evolution);
            write_evolution_header(evolution_file);
            evolution_writer = &evolution_file;
        }
        if (!args.save_states.empty()) {
            states_file = open_output_file(args.save_states);
            write_states_header(states_file);
            states_writer = &states_file;
        }

        write_evolution_record(evolution_writer, 0, system);
        write_state_records(states_writer, 0, system);
        for (std::size_t step = 1; step <= args.num_steps; ++step) {
            system.update(args.delta_time);
            write_evolution_record(evolution_writer, step, system);
            write_state_records(states_writer, step, system);
        }
    } catch (const std::exception& error) {
        std::cerr << "error: " << error.what() << '\n';
        return 1;
    }
    return 0;
}
