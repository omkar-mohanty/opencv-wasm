use super::ffi;
use cxx::{let_cxx_string, UniquePtr, CxxVector};
pub fn imread(filename: &'static str, flags: i32) -> cxx::UniquePtr<ffi::Mat> {
    let_cxx_string!(filename = filename);
    let mat;

    mat = ffi::imread(&filename, flags);

    mat
}

pub fn imwrite(filename: &'static str, img:UniquePtr<ffi::InputArray>, params:&CxxVector<i32>) {
    let_cxx_string!(filename = filename);
    ffi::imwrite(&filename, img, params);
}
