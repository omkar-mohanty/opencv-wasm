pub mod imgcodecs;
pub mod imgproc;
#[cxx::bridge(namespace = "manual")]
mod ffi {
    unsafe extern "C++" {
        include!("opencv-wasm/include/imgcodecs.hpp");
        include!("opencv-wasm/include/imgproc.hpp");
        type Mat;
        type InputArray;
        type OutputArray;
        type Point;
        type Size;
        fn imread(filename: &CxxString, flags: i32) -> UniquePtr<Mat>;

        fn imwrite(
            filename: &CxxString,
            img: UniquePtr<InputArray>,
            params: &CxxVector<i32>,
        ) -> bool;

        fn bilateralFilter(
            src: UniquePtr<InputArray>,
            dst: UniquePtr<OutputArray>,
            d: i32,
            sigmaColor: f64,
            sigmaSpace: f64,
            borderType: i32,
        );

        fn blur(
            src: UniquePtr<InputArray>,
            dst: UniquePtr<OutputArray>,
            ksize: UniquePtr<Size>,
            anchor: UniquePtr<Point>,
            borderType: i32,
        );
    }
}
