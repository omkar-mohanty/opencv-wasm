use cxx::{let_cxx_string, UniquePtr};

/// This module is currently unsupported 
use super::ffi;

pub fn named_window(winname: &'static str, flags: i32) {
    let_cxx_string!(winname = winname);

    ffi::namedWindow(&winname, flags);
}

pub fn imshow(winname: &'static str, mat: UniquePtr<ffi::Mat>) {
    let_cxx_string!(winname = winname);

    ffi::imshow(&winname, mat);
}
