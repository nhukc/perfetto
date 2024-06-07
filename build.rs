use cmake::Config;
use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // Define the Perfetto repository URL.
    let repo_url = "https://android.googlesource.com/platform/external/perfetto/";

    // Define the target directory.
    let target_dir = PathBuf::from(env::var("OUT_DIR").unwrap()).join("perfetto");

    // Clone the Perfetto repository if it doesn't exist.
    if !target_dir.exists() {
        let status = Command::new("git")
            .arg("clone")
            .arg(repo_url)
            .arg("-b")
            .arg("v45.0")
            .arg(&target_dir)
            .status()
            .expect("Failed to clone Perfetto repository");
        if !status.success() {
            panic!("Git clone command failed");
        }
    }

    // Change to the Perfetto directory.
    let perfetto_dir = target_dir.clone();

    // Run tools/install-build-deps.
    let status = Command::new("bash")
        .arg("-c")
        .arg("./tools/install-build-deps")
        .current_dir(&perfetto_dir)
        .status()
        .expect("Failed to run install-build-deps");
    if !status.success() {
        panic!("Install build dependencies command failed");
    }

    // Run tools/gn gen --args='is_debug=false' out/linux.
    let status = Command::new("bash")
        .arg("-c")
        .arg("tools/gn gen --args='is_debug=false' out/linux")
        .current_dir(&perfetto_dir)
        .status()
        .expect("Failed to run GN gen");
    if !status.success() {
        panic!("GN gen command failed");
    }

    // Run ninja build.
    let status = Command::new("bash")
        .arg("-c")
        .arg("tools/ninja -C out/linux tracebox traced traced_probes perfetto")
        .current_dir(&perfetto_dir)
        .status()
        .expect("Failed to run Ninja build");
    if !status.success() {
        panic!("Ninja build command failed");
    }

    // Configure and build the C++ project using CMake.
    let dst = Config::new("cmake")
        .define("PERFETTO_DIR", perfetto_dir.display().to_string())
        .define("CMAKE_BUILD_TYPE", "Release")
        .build_target("wrapper")
        .build();

    // Print the build directory for debugging.
    println!("Build directory: {}", dst.display());

    // Tell Cargo to link the built static library.
    let lib_path = PathBuf::from(dst).join("build");
    println!("cargo:rustc-link-search=native={}", lib_path.display());
    println!("cargo:rustc-link-lib=static=wrapper");

    // Rebuild if the C++ source files change.
    println!("cargo:rerun-if-changed=cmake/wrapper.cpp");

    // Link C++ standard libary.
    println!("cargo:rustc-link-lib=dylib=stdc++");

}

