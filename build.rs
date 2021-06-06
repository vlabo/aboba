fn main() {
    cc::Build::new()
        .file("src/c/player.c")
        .flag("-I/usr/include/gstreamer-1.0")
        .flag("-I/usr/include/x86_64-linux-gnu")
        .flag("-I/usr/include/glib-2.0")
        .flag("-I/usr/lib/x86_64-linux-gnu/glib-2.0/include")
        .compile("player");


        println!("cargo:rustc-link-lib=dylib=gstreamer-1.0");
}

