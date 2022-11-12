use super::{
    ffi,
    ffi::{InputArray, OutputArray, Point, Size},
};
use cxx::UniquePtr;

pub fn bilateral_filter(
    src: UniquePtr<InputArray>,
    dst: UniquePtr<OutputArray>,
    d: i32,
    sigma_color: f64,
    sigma_space: f64,
    border_type: i32,
) {
    ffi::bilateralFilter(src, dst, d, sigma_color, sigma_space, border_type);
}

pub fn blur(
    src: UniquePtr<InputArray>,
    dst: UniquePtr<OutputArray>,
    ksize: UniquePtr<Size>,
    anchor: UniquePtr<Point>,
    border_type: i32,
) {
    ffi::blur(src, dst, ksize, anchor, border_type);
}

pub fn box_filter(
    src: UniquePtr<InputArray>,
    dst: UniquePtr<OutputArray>,
    ddepth: i32,
    ksize: UniquePtr<Size>,
    anchor: UniquePtr<Point>,
    normalize: bool,
    border_type: i32,
) {
    ffi::boxFilter(src, dst, ddepth, ksize, anchor, normalize, border_type);
}
