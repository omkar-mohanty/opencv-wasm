use opencv_wasm::{imgcodecs::{imread, imwrite},Mat};
use std::{env, fs::File};
fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = args[1].clone();

    match imread(filename, 1) {
        Err(msg) => println!("{}", msg),

        Ok(mat) => {
            let dst = Mat::new();

            //let ksize = Size::new(5, 5);

            //let anchor = Point::new(1, 1);

            //blur(mat, dst, ksize, anchor, 1);

            File::create("blur.jp2").expect("Error could not create blur.jp2");

            imwrite("blur.jp2", mat);
        }
    }
}
