use super::{ffi, CvError};
use cxx::{let_cxx_string, CxxVector, UniquePtr};
use std::error::Error;

pub fn imread(
    filename: String,
    flags: i32,
) -> Result<cxx::UniquePtr<ffi::Mat>, Box<dyn Error>> {
    let filename = filename.as_str();

    if !std::path::Path::new(filename).exists() {
        return Err(Box::new(CvError::new("file not found")));
    }

    let_cxx_string!(filename = filename);

    let mat;

    mat = ffi::imread(&filename, flags);

    Ok(mat)
}

pub fn imwrite(filename: &'static str, img: UniquePtr<ffi::InputArray>, params: &CxxVector<i32>) {
    let_cxx_string!(filename = filename);
    ffi::imwrite(&filename, img, params);
}
