fn main() {
    println!("cargo::rerun-if-changed=native/interpolator.hpp");
    println!("cargo::rerun-if-changed=native/interpolator.cpp");
    println!("cargo::rerun-if-changed=native/interpolator_c.h");
    println!("cargo::rerun-if-changed=native/interpolator_c.cpp");

    cc::Build::new()
        .cpp(true)
        .std("c++17")
        .warnings(true)
        .extra_warnings(true)
        .include("native")
        .file("native/interpolator.cpp")
        .file("native/interpolator_c.cpp")
        .compile("interpolation");
}
