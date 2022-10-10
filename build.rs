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

fn main() -> miette::Result<()> {
    let include = std::path::PathBuf::from("./include");
    let opencv = std::path::PathBuf::from("/usr/include/opencv4");
    let sysroot = std::path::PathBuf::from("/opt/wasi-sdk/share/sysroot");
    let mut b = autocxx_build::Builder::new("src/lib.rs", &[&include, &opencv, &sysroot]).build()?;

    b.flag_if_supported("-std=c++17")
        .flag("--sysroot=/opt/wasi-sdk/share/sysroot")
        .cpp(true)
        .compile("ocvrs-wasm");

    let lib = pkg_config::Config::new().probe("opencv4").unwrap();

    lib.ld_args.iter().for_each(|args| {
        args.iter()
            .for_each(|link| println!("cargo-rustc-link-lib={}", link))
    });

    Ok(())
}
