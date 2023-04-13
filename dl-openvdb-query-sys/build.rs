fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build bindings
    let bindings = bindgen::Builder::default()
        .header("include/wrapper.hpp")
        // Searchpath
        .clang_arg("-Iinclude")
        .clang_arg("-xc++")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        //.parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Could not write bindings.");

    Ok(())
}
