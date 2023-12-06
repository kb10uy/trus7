fn main() {
    cc::Build::new()
        .file("tr7.c")
        .include("./")
        .define("VERSION", "\"1.0.8\"")
        .define("ssize_t", "intptr_t")
        .std("c11")
        .compile("tr7");
}
