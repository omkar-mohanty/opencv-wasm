fn main() -> Result<(), Box<dyn std::error::Error>> {
    let libs: Vec<&str> = vec![
        "opencv_highgui",
        "opencv_imgproc",
        "opencv_imgcodecs",
        "opencv_core",
        "c++",
        "c++abi",
        "webp",
        "openjp2"
    ];

    println!("cargo:rustc-link-search=lib");
    //let lib = pkg_config::Config::new().probe("opencv4").unwrap();

    let source_files = vec!["src/lib.rs"];
    let mut bridge = cxx_build::bridges(source_files);
    /*lib.ld_args.iter().for_each(|args| {
        args.iter()
            .for_each(|link| println!("cargo-rustc-link-lib={}", link))
    });*/

    for lib in libs {
        println!("cargo:rustc-link-lib=static={}", lib);
    }

    bridge
        .include("/usr/local/include")
        .include("/usr/local/include/opencv4")
        .compile("ocvrs-wasm");

    Ok(())
}
