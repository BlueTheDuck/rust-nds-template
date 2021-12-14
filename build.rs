fn main() {
    let devkitarm = match std::env::var("DEVKITARM") {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Error: DEVKITARM environment variable not set. 
            Are you sure devkitArm is installed? Please check https://devkitpro.org/wiki/Getting_Started 
            $DEVKITARM/bin/ should be in your path, and contain arm-none-eabi-gcc");
            panic!("DEVKITARM not found");
        }
    };
    let devkitarm = std::path::PathBuf::from(devkitarm);
    let gcc = devkitarm.join("bin/arm-none-eabi-gcc");
    assert!(gcc.exists(), "arm-none-eabi-gcc not found at {:?}", gcc);
    cc::Build::new()
        .compiler("arm-none-eabi-gcc")
        .file("src/ferris.s")
        .compile("ferris");
}
