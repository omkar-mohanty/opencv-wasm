pub mod highgui;
pub mod imgcodecs;
pub mod imgproc;

use cxx::UniquePtr;
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

pub struct Mat {
    pub mat: UniquePtr<ffi::Mat>,
}

impl Mat {
    pub fn new() -> Self {
        Self { mat: ffi::mat() }
    }
}

impl Default for Mat {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Size {
    pub size: UniquePtr<ffi::Size>,
}

impl Size {
    pub fn new(i: i32, j: i32) -> Self {
        Self {
            size: ffi::size(i, j),
        }
    }
}

pub struct Point {
    pub point: UniquePtr<ffi::Point>,
}

impl Point {
    pub fn new(i: i32, j: i32) -> Self {
        Self {
            point: ffi::point(i, j),
        }
    }
}

#[cxx::bridge(namespace = "manual")]
mod ffi {
    unsafe extern "C++" {
        include!("opencv-wasm/include/imgcodecs.hpp");
        type Mat;
        type Point;
        type Size;
        fn imread(filename: &CxxString, flags: i32) -> UniquePtr<Mat>;

        fn imwrite(filename: &CxxString, img: UniquePtr<Mat>) -> bool;

        fn bilateralFilter(
            src: UniquePtr<Mat>,
            dst: UniquePtr<Mat>,
            d: i32,
            sigmaColor: f64,
            sigmaSpace: f64,
            borderType: i32,
        );

        fn blur(
            src: UniquePtr<Mat>,
            dst: UniquePtr<Mat>,
            ksize: UniquePtr<Size>,
            anchor: UniquePtr<Point>,
            borderType: i32,
        );

        fn boxFilter(
            src: UniquePtr<Mat>,
            dst: UniquePtr<Mat>,
            ddepth: i32,
            ksize: UniquePtr<Size>,
            anchor: UniquePtr<Point>,
            normalize: bool,
            borderType: i32,
        );

        fn namedWindow(winname: &CxxString, flags: i32);

        fn imshow(winname: &CxxString, mat: UniquePtr<Mat>);

        fn mat() -> UniquePtr<Mat>;

        fn size(i: i32, j: i32) -> UniquePtr<Size>;

        fn point(i: i32, j: i32) -> UniquePtr<Point>;
    }
}
