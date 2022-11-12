use std::env::set_var; 

const WASI_SDK_PATH:&str = "/opt/wasi-sdk";
const WASI_SYSROOT:&str = "/opt/wasi-sdk/share/wasi-sysroot";

fn prepeare_env() {
    let cc = format!("{}/bin/clang --sysroot={}",WASI_SDK_PATH,WASI_SYSROOT);
    let cxx = format!("{}/bin/clang++ --sysroot={}", WASI_SDK_PATH,WASI_SYSROOT );
    let ar = format!("{}/bin/llvm-ar",WASI_SDK_PATH);
    let cxx_wasm32_wasi = cxx.clone();
    let cc_wasm32_wasi = cc.clone();
    let cargo_target_wasm32_wasi_linker = format!("{}/bin/wasm-ld", WASI_SDK_PATH);

    set_var("WASI_SDK_PATH", WASI_SDK_PATH);
    set_var("WASI_SYSROOT", WASI_SYSROOT);
    set_var("CC", cc);
    set_var("CXX", cxx);
    set_var("AR", ar);
    set_var("CXX_wasm32_wasi", cxx_wasm32_wasi);
    set_var("CC_wasm32_wasi", cc_wasm32_wasi);
    set_var("CARGO_TARGET_WASM32_WASI_LINKER", cargo_target_wasm32_wasi_linker);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    prepeare_env();
    let libs:Vec<&str> = vec![
        //"opencv_highgui",
        "opencv_imgproc",
        "opencv_imgcodecs",
        "opencv_core",
        "c++",
        "c++abi"
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

    let sysroot = format!("--sysroot={}",WASI_SYSROOT);

    bridge
        .flag(&sysroot)    
        .include("/usr/local/include")
        .include("/usr/local/include/opencv4")
        .compile("ocvrs-wasm");

    Ok(())
}
