

fn main() {
    let dst = cmake::Config::new("CLBlast")
      .define("BUILD_SHARED_LIBS", "OFF")
      .build();
    // Tell cargo to tell rustc to link the clblast library
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=clblast");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");
    
    #[cfg(feature = "bindgen")] {
    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
            // The input header we would like to generate
            // bindings for.
            .header("wrapper.hpp")
            .clang_arg("-std=c++03")
            .rustified_enum("CLBlast.*")
            .generate_block(false)
            .size_t_is_usize(true)
            .ctypes_prefix("::libc")
            // Tell cargo to invalidate the built crate whenever any of the
            // included header files changed.
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            // Finish the builder and generate the bindings.
            .generate()
            // Unwrap the Result and panic on failure.
            .expect("Unable to generate bindings");
        
        bindings
            .write_to_file("src/bindings.rs")
            .expect("Couldn't write bindings!");
    }
    
    if cfg!(target_os = "windows") {
        // nothing
    }
    else if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=c++");
    }
    else {
        println!("cargo:rustc-link-lib=stdc++");
    };
    
    println!("cargo:include={}", dst.join("include").display().to_string());
}
