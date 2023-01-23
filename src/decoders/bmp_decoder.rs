use std::path::Path;

use crate::utils::image::{self, Image};
use crate::utils::pixel::Pixel;

pub fn open(path : &Path) -> Image{
    let data: Vec<u8> = std::fs::read(path).unwrap();

    let file_header_bytes = &data[0..14];

    println!("Actual file size in bytes is: {}", data.len());

    // Check if file is a BMP file
    if file_header_bytes[0] != 0x42 || file_header_bytes[1] != 0x4D {
        // Error not a BMP file
        panic!("Image is not a BMP file.");
    }

    // Read file size from header
    let header_filesize_slice: [u8;4] = file_header_bytes[2..6].try_into().unwrap();
    let header_filesize: u32 = u32::from_le_bytes(header_filesize_slice);
    println!("Header said file size is {}.", header_filesize);

    // Get offset of pixel data
    let offset_slice: [u8;4] = file_header_bytes[10..14].try_into().unwrap();
    let offset: u32 = u32::from_le_bytes(offset_slice);
    println!("Header said pixel data offset is {}.", offset);

    // Check DIB
    let dib_header_size: u32 = u32::from_le_bytes(data[14..18].try_into().unwrap());
    print!("DIB header size is {}.", dib_header_size);

    if dib_header_size == 40 {
        println!(" DIB header is of type BITMAPINFOHEADER.")
    }
    if dib_header_size == 124 {
        println!(" DIB header is of type BITMAPV5HEADER.")
    }

    // Get picture size
    let width: u32 = u32::from_le_bytes(data[18..22].try_into().unwrap());
    let height: u32 = u32::from_le_bytes(data[22..26].try_into().unwrap());

    println!("Width is: {}. height is: {}", width, height);

    // Get bits per pixel
    let bits_per_pixel: u16 = u16::from_le_bytes(data[28..30].try_into().unwrap());
    println!("Bits per pixel: {}", bits_per_pixel);

    // Get compression method
    let comp_type: u32 = u32::from_le_bytes(data[30..34].try_into().unwrap());

    if comp_type == 0 {
        println!("No Compression is used.");
    }
    if comp_type != 0 {
        panic!("Only uncompressed BMPs are supported.");
    }

    // Image data size
    let image_data_size: u32 = u32::from_le_bytes(data[34..38].try_into().unwrap());
    println!("Size of image data: {}", image_data_size);

    // Pixel data
    let row_size = (bits_per_pixel as u32 * width + 31) / 32 * 4;

    println!("Size of row: {}", row_size);
    //println!("row_size * height = {}", row_size * height);

    let pixel_data: Vec<u8> = data[offset as usize ..(row_size * height + offset) as usize].to_vec();

    //println!("Pixel data slice len is: {}", pixel_data.len());

    let mut pixels: Vec<Pixel<u8>> = Vec::new();

    // Loop though all pixels in image
    for pix_y in (0..height).rev() { // BMP starts on the bottom row
        for pix_x in 0..width {
            let offset: u32 = pix_y * row_size + (pix_x * 3);
            let pixel = Pixel::<u8> {
                b: pixel_data[offset as usize], // order is bgr not rgb
                g: pixel_data[(offset + 1) as usize], 
                r: pixel_data[(offset + 2) as usize],};
            
            pixels.push(pixel);
        }
    }
    println!("Total pixels decoded: {}", pixels.len());

    let image = image::Image{height: height, width: width, pixels: pixels};
    
    return image;

}