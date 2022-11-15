use super::{ffi, Mat, Point, Size};

pub fn bilateral_filter(
    src: Mat,
    dst: Mat,
    d: i32,
    sigma_color: f64,
    sigma_space: f64,
    border_type: i32,
) {
    ffi::bilateralFilter(src.mat, dst.mat, d, sigma_color, sigma_space, border_type);
}

pub fn blur(src: Mat, dst: Mat, ksize: Size, anchor: Point, border_type: i32) {
    ffi::blur(src.mat, dst.mat, ksize.size, anchor.point, border_type);
}

pub fn box_filter(
    src: Mat,
    dst: Mat,
    ddepth: i32,
    ksize: Size,
    anchor: Point,
    normalize: bool,
    border_type: i32,
) {
    ffi::boxFilter(
        src.mat,
        dst.mat,
        ddepth,
        ksize.size,
        anchor.point,
        normalize,
        border_type,
    );
}
