extern crate pkg_config;

const _LIBS: [&str; 16] = [
    "opencv_photo",
    "opencv_objdetect",
    "opencv_highgui",
    "opencv_imgproc",
    "opencv_imgcodecs",
    "opencv_flann",
    "opencv_videoio",
    "opencv_video",
    "opencv_ts",
    "opencv_dnn",
    "opencv_gapi",
    "opencv_features2d",
    "opencv_stitching",
    "opencv_ml",
    "opencv_core",
    "opencv_calib3d",
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rustc-link-search=lib/");
    let lib = pkg_config::Config::new().probe("opencv4").unwrap();

    let mut bridge = cxx_build::bridge("src/lib.rs");

    lib.ld_args.iter().for_each(|args| {
        args.iter()
            .for_each(|link| println!("cargo-rustc-link-lib={}", link))
    });
    

    bridge
        .include("/usr/local/include")
        .include("/usr/local/include/opencv4")
        .flag("--sysroot=/opt/wasi-sdk/share/sysroot")
        .flag("-std=c++17")
        .cpp(true)
        .compile("ocvrs-wasm");

    Ok(())
}
