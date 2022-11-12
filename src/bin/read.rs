use opencv_wasm::imgcodecs::imread;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = String::from(args[1].clone());
    if let Err(msg) = imread(filename, 1) {
        println!("{}", msg)
    }
}
