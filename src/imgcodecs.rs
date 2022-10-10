use crate::ffi::{cv, manual};
use autocxx::c_int;
use cxx::{let_cxx_string, UniquePtr};

type Mat = UniquePtr<cv::Mat>;

pub fn imread(filename: &'static str, flags: i32) -> Mat {
    let_cxx_string!(filename = filename);

    manual::imread(filename, c_int(flags))
}

pub fn imreadmulti(filename:&'static str, mats: std::pin::Pin<&mut cxx::CxxVector<cv::Mat>>,flags: i32) -> bool {
    let_cxx_string!(filename = filename);
    cv::imreadmulti(&filename, mats, c_int(flags))
}

pub fn imwrite() {
    
}
