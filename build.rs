//use std::path::Path;
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

const LIBS: [&str; 16] = [
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

    println!("cargo:rustc-link-search=/opt/wasi-sdk/share/sysroot/lib/wasm32-wasi");
    println!("cargo:rustc-link-search=/usr/local/lib");
    //let lib = pkg_config::Config::new().probe("opencv4").unwrap();

    let mut bridge = cxx_build::bridge("src/lib.rs");

    /*lib.ld_args.iter().for_each(|args| {
        args.iter()
            .for_each(|link| println!("cargo-rustc-link-lib={}", link))
    });*/
    
    /*for lib in LIBS {
        println!("cargo-rustc-link-lib={}", lib);
    }*/    

    bridge
        .include("/usr/local/include")
        .cpp(true)
        .include("/usr/local/include/opencv4")
        .compile("ocvrs-wasm");

    Ok(())
}
