pub mod imgcodecs;
pub mod imgproc;
use std::{error::Error, fmt::Display};

#[derive(Debug)]
struct CvError<'a> {
    msg: &'a str,
}

impl<'a> CvError<'a> {
    pub fn new(msg: &'a str) -> Self {
        CvError { msg }
    }
}

impl<'a> Display for CvError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl<'a> Error for CvError<'a> {}

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

        fn boxFilter(
            src: UniquePtr<InputArray>,
            dst: UniquePtr<OutputArray>,
            ddepth: i32,
            ksize: UniquePtr<Size>,
            anchor: UniquePtr<Point>,
            normalize: bool,
            borderType: i32,
        );
    }
}
