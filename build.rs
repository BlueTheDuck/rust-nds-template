fn main() -> Result<(), Box<dyn std::error::Error>> {
    let toolchain = match std::env::var("WONDERFUL_TOOLCHAIN") {
        Ok(v) => v,
        Err(_) => {
            eprintln!(
                "Error: WONDERFUL_TOOLCHAIN environment variable not set. 
            Are you sure BLOCKSDS and WONDERFUL_TOOLCHAIN are installed?"
            );
            panic!("toolchain not found");
        }
    };
    let toolchain = std::path::PathBuf::from(toolchain);
    let gcc = toolchain.join("toolchain/gcc-arm-none-eabi/bin/arm-none-eabi-gcc");
    assert!(gcc.exists(), "arm-none-eabi-gcc not found at {gcc:?}");

    println!("cargo:rerun-if-changed=assets");
    println!("cargo:rerun-if-changed=build.rs");

    Ok(())
}
