extern crate pkg_config;
const _LIBS: [&str; 16] = [
    "libopencv_photo.a",
    "libopencv_objdetect.a",
    "libopencv_highgui.a",
    "libopencv_imgproc.a",
    "libopencv_imgcodecs.a",
    "libopencv_flann.a",
    "libopencv_videoio.a",
    "libopencv_video.a",
    "libopencv_ts.a",
    "libopencv_dnn.a",
    "libopencv_gapi.a",
    "libopencv_features2d.a",
    "libopencv_stitching.a",
    "libopencv_ml.a",
    "libopencv_core.a",
    "libopencv_calib3d.a",
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rustc-link-search=/opt/wasi-sdk/share/sysroot/lib/wasm32-wasi");
    println!("cargo:rustc-link-search=/usr/local/lib");
    //let lib = pkg_config::Config::new().probe("opencv4").unwrap();

    let source_files = vec!["src/lib.rs"];
    let mut bridge = cxx_build::bridges(source_files);
    /*lib.ld_args.iter().for_each(|args| {
        args.iter()
            .for_each(|link| println!("cargo-rustc-link-lib={}", link))
    });*/

    /*for lib in LIBS {
        println!("cargo-rustc-link-lib={}", lib);
    }*/

    bridge
        .archiver("llvm-ar")
        .cpp_link_stdlib(None)
        .cpp(true)
        .include("/usr/local/include")
        .include("/usr/local/include/opencv4")
        .compile("ocvrs-wasm");

    Ok(())
}
