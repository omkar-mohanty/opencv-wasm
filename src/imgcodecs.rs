
use cxx::let_cxx_string;
pub fn imread(filename: &'static str, flags: i32) -> cxx::UniquePtr<ffi::Mat> {
    let_cxx_string!(filename = filename);
    let mat;

    mat = ffi::imread(&filename, flags);

    mat
}
