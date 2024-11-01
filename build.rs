use miette::IntoDiagnostic;

fn main() -> miette::Result<()> {
    let mut cmake_config = cmake::Config::new("third_party/cpuinfo");

    match std::env::var("CARGO_CFG_TARGET_OS").unwrap().as_str() {
        "android" => {
            cmake_config
                .define("CMAKE_ANDROID_NDK", std::env::var("ANDROID_NDK_HOME").into_diagnostic()?);
        }
        "ios" => {
            cmake_config.define("CMAKE_SYSTEM_NAME", "Darwin");
        }
        _ => {
            cmake_config
                .define("CPUINFO_BUILD_BENCHMARKS", "OFF")
                .define("CPUINFO_BUILD_MOCK_TESTS", "OFF")
                .define("CPUINFO_BUILD_PKG_CONFIG", "OFF")
                .define("CPUINFO_BUILD_TOOLS", "OFF")
                .define("CPUINFO_BUILD_UNIT_TESTS", "OFF")
                .define("CPUINFO_LIBRARY_TYPE", "static")
                .define("CPUINFO_RUNTIME_TYPE", "static")
                .define("USE_SYSTEM_LIBS", "OFF");
        }
    }

    let cmake_output_path = cmake_config.build();

    println!("cargo:rustc-link-lib=static=cpuinfo");
    println!("cargo:rustc-link-search=native={}/lib", cmake_output_path.display());

    Ok(())
}
