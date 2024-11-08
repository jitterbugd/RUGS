use image::{GenericImageView, ImageBuffer}; // For testing
use show_image::{ImageView, ImageInfo, create_window}; // For testing
use flate2::{Compression, write::{ZlibEncoder, ZlibDecoder}}; 

use std::{
    hash::Hash,
    io::{self, Write},
    collections::HashMap
};

/* 
   
*/

mod compressor;
mod implementation;
use implementation::Rgba;
use implementation::Image;

fn breakpoint() {
    println!("[?] Press 'ENTER' when you're ready to continue.");
    io::stdin().read_line(&mut String::new()).unwrap();
}

const RUGS_MAGIC_BYTES: &[u8; 4] = b"RUGS";
const HEADER_COUNT: usize = 3;

#[show_image::main]
fn main() {

    let img = image::open("artifacts/test.png").unwrap();

    let (width, height) = img.dimensions();
    println!("Width: {}, Height: {}", width, height);

    let rgba_image: ImageBuffer<image::Rgba<u8>, Vec<u8>> = img.to_rgba8();
    let raw_image_data = rgba_image.into_raw();

    let mut image_data: Vec<Rgba> = vec![];
    for chunk in raw_image_data.chunks(4) {
        image_data.push(Rgba::from_bytes(chunk));
    }

    let mut new_image = Image {
        width,
        height,
        image_data,
        lossy_compressed: false
    };

    if let Err(e) = new_image.lossy_compress(implementation::ComperssionAmnt::MED) {
        panic!("[-] Lossy compression failed: {}", e)
    };

    std::fs::write("artifacts/output.rugs", new_image.deserialize()).unwrap();

    breakpoint();

    let file_data: Vec<u8> = std::fs::read("artifacts/output.rugs").unwrap();
    let new_image = match implementation::Image::serialize(file_data) {
        Ok(image) => image,
        Err(e) => panic!("[-] Unable to serialize image: {}", e),
    };
        
    let image_bytes = new_image.image_bytes();
    let image = ImageView::new(ImageInfo::rgba8(new_image.width, new_image.height), &image_bytes);
    let window = create_window("Image", Default::default()).unwrap();
    window.set_image("Image", image).unwrap();
    
    breakpoint();
}
