pub mod imgcodecs;
#[cxx::bridge(namespace = "manual")]
mod ffi {
    unsafe extern "C++" {
        include!("opencv-wasm/include/manual.hpp");

        type Mat;

        fn imread(filename: &CxxString, flags: i32) -> UniquePtr<Mat>;

    }
}

