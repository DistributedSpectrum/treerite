fn main() {
    // Prefer a prebuilt treelite static lib: Nix's build/treelite.nix compiles
    // treelite once and points us here via TREELITE_STATIC_DIR. Fall back to
    // compiling the vendored submodule in-tree so a plain `cargo build` (no Nix)
    // still works.
    match std::env::var("TREELITE_STATIC_DIR") {
        Ok(dir) => {
            println!("cargo:rustc-link-search={dir}");
        }
        Err(_) => {
            let dst = cmake::Config::new("treelite")
                .define("Treelite_BUILD_STATIC_LIBS", "ON")
                .define("Treelite_USE_DYNAMIC_MSVC_RUNTIME", "ON")
                .define("Treelite_INSTALL", "OFF")
                .build();
            println!("cargo:rustc-link-search={}/build", dst.display());
        }
    }
    println!("cargo:rustc-link-lib=static=treelite_static");
    #[cfg(target_os = "linux")]
    {
        println!("cargo:rustc-link-lib=gomp");
        println!("cargo:rustc-link-lib=stdc++");
    }
    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-lib=omp");
        println!("cargo:rustc-link-lib=c++");
    }
}
