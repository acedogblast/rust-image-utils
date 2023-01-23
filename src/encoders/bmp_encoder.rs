use std::path::Path;
pub fn write(path : &Path, image : &Image) {
    let mut file = File::create(path);
    let mut size : u32 = 14 + 40; // size of file header and BITMAPINFOHEADER

    let image_array_size: u32 = (bits_per_pixel as u32 * width + 31) / 32 * 4;;

    let bmp_header: Vec<u8> = [
        40u32.to_le_bytes(),       // Size of header
        image.width.to_le_bytes(), // Image width
        image.height.to_le_bytes(),// Image height
        1u16.to_le_bytes(),        // Color plains always 1
        24u16.to_le_bytes(),       // Bits per pixel, 24 for this program
        0u32.to_le_bytes,          // Compression used. 0 = none.

    ];



    let header: [u8;14] = [
        0x42,0x4d, // File singature "BM"
        0x00,0x00,0x00,0x00, // Total size of file le 4 bytes
        0x00,0x00,0x00,0x00, // Reserved
        0x00,0x00,0x00,0x00  // Offset to pixel array
        ];



}