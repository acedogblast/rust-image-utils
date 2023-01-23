
use std::path::Path;

mod decoders {pub mod bmp_decoder;}
mod utils {pub mod image; pub mod pixel;}

use crate::decoders::bmp_decoder;

fn main() {
    println!("Jpeg and BMP in Rust!");

    let path  = Path::new("star_trek_star_trek_voyager.bmp");

    let image = bmp_decoder::open(path);

    println!("The rng value of pixel (12,50) is {}", image.get_pixel(12,50));

}
