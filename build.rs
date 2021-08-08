fn main() {
    let gstreamer = pkg_config::Config::new().probe("gstreamer-1.0").unwrap();

    let src = ["src/audio/c/player.c"];

    let mut builder = cc::Build::new();
    let build = builder.files(src.iter());

    for inc in gstreamer.include_paths {
        build.include(inc);
    }

    build.compile("player");

    for ipath in gstreamer.link_paths {
        println!("cargo:rustc-link-search=dylib={}", ipath.to_str().unwrap());
    }

    for lib in gstreamer.libs {
        println!("cargo:rustc-link-lib=dylib={}", lib.as_str());
    }
}
