fn main() {
    cc::Build::new()
        .std("c11")
        .flag_if_supported("-FC")
        .define("VERSION", "\"1.0.8\"")
        .file("./tr7/tr7.c")
        .include("./tr7")
        .compile("tr7");
}
