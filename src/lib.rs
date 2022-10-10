pub mod imgcodecs;
use autocxx::prelude::*;

include_cpp! {
    #include "manual.hpp"
    safety!(unsafe)
    generate!("manual::imread")
    generate!("cv::imreadmulti")
    generate!("cv::imwrite")
}
