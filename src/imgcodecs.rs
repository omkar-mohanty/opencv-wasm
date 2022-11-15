use super::{ffi, CvError, Mat};
use cxx::let_cxx_string;
use std::error::Error;

pub fn imread(filename: String, flags: i32) -> Result<Mat, Box<dyn Error>> {
    let filename = filename.as_str();

    if !std::path::Path::new(filename).exists() {
        return Err(Box::new(CvError::new("file not found")));
    }

    let_cxx_string!(filename = filename);

    let mat = ffi::imread(&filename, flags);

    Ok(Mat { mat })
}

pub fn imwrite(filename: &'static str, img: Mat) {
    let_cxx_string!(filename = filename);
    ffi::imwrite(&filename, img.mat);
}
