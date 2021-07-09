fn main() {
    cc::Build::new()
        .compiler("arm-none-eabi-gcc")
        .file("src/ferris.s")
        .compile("ferris");
}