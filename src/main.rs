use image::{GenericImageView, ImageBuffer}; // For testing
use show_image::{ImageView, ImageInfo, create_window}; // For testing
use flate2::{Compression, write::{ZlibEncoder, ZlibDecoder}};
use std::env;

use std::{
    hash::Hash,
    io::{self, Write},
    collections::HashMap
};

mod compressor;
mod tests;
mod implementation;
use implementation::Rgba;
use implementation::Image;
use implementation::ComperssionAmnt;

fn graceful_exit(arg_names: &Vec<&str>, arg_descs: &Vec<&str>) {
    for (arg_name, arg_desc) in arg_names.iter().zip(arg_descs.iter()) {
        println!("{arg_name:<18} | {arg_desc}")
    }
    println!("\nExample usage: \n\t./RUGS --compression_type=none --input=example.png --output=completed.rugs --view=true");
    std::process::exit(1);

}

#[show_image::main] // Required for show_image library
fn main() {
    let (arg_names, arg_descs) = (vec![
        "view",
        "compression",
        "output",
        "input"
    ], vec![
        "View the image upon completion (Default is false)",
        "Level of lossy compression (ultra, high, med, min, none) (Default is none)",
        "Path to output generated file (Default is \"output.rugs\")",
        "Path to input PNG file for conversion (Required)"
    ]);

    let args: Vec<String> = env::args().collect();

    let mut arguments_map: HashMap<String, Option<String>> = HashMap::new();
    for arg_name in &arg_names {arguments_map.insert(arg_name.to_string(), None);}
    
    // Print out help if none were supplied
    if args.len() == 1 {graceful_exit(&arg_names, &arg_descs);}

    for arg in args.iter().filter(|arg| arg.starts_with("--")) {
        let parts: Vec<&str> = arg.split('=').collect();

        let argument_name = parts[0].replace("--", "");
        let argument_value =  parts.get(1).map(|s| s.to_string()); // Convert from &&str to String

        if argument_value.is_none() {
            println!("\"{}\" cannot have a blank value\n", argument_name);
            graceful_exit(&arg_names, &arg_descs);
        }

        // Check if arugment is in argument list and also handle a blank value
        if !arg_names.contains(&argument_name.as_str()) {
            println!("\"{}\" isn't a valid argument\n", argument_name);
            graceful_exit(&arg_names, &arg_descs);
        }
        
        arguments_map.insert(argument_name, argument_value);
    }

    let input_path = match arguments_map.get("input").unwrap() {
        None => {
            println!("You didn't specify the input path.\n");
            graceful_exit(&arg_names, &arg_descs); unreachable!()
        },
        Some(result) => result
    };

    let output_path = match arguments_map.get("output").unwrap() {
        None => "output.rugs",
        Some(result) => result,
    };

    let compression_magnitude = match arguments_map.get("compression").unwrap() {
        None => ComperssionAmnt::NONE,
        Some(result) => {
            match result.to_ascii_lowercase().as_str() {
                "ultra" => ComperssionAmnt::ULTRA,
                "high" => ComperssionAmnt::HIGH,
                "med" => ComperssionAmnt::MED,
                "min" => ComperssionAmnt::MIN,
                "none" => ComperssionAmnt::NONE,
                &_ => {
                    println!("You didn't specify one of the supported compression types.\n");
                    graceful_exit(&arg_names, &arg_descs); unreachable!()
                }
            }
        },
    };

    let start_time = std::time::SystemTime::now();
    println!("Your settings seem correct! Starting the conversion ...\n");

    let img = image::open(input_path).unwrap_or_else(|error| {
        println!("Unable to open input image: {:?}", error);
        std::process::exit(1);
    });
    
    let (width, height) = img.dimensions();
    let rgba_image: ImageBuffer<image::Rgba<u8>, Vec<u8>> = img.to_rgba8();

    let mut image_data: Vec<Rgba> = vec![];
    for chunk in rgba_image.into_raw().chunks(4) {
        image_data.push(Rgba::from_bytes(chunk));
    }

    let mut new_rugs_image = Image {
        width,
        height,
        image_data,
        lossy_compressed: false // placeholder
    };

    if let Err(error) = new_rugs_image.lossy_compress(compression_magnitude) {
        println!("[-] Unable to lossy compress image: {:?}", error);
        std::process::exit(1);
    }

    let rugs_image_bytes = &new_rugs_image.deserialize();
    if let Err(error) = std::fs::write(output_path, rugs_image_bytes) {
        println!("[-] Unable to write image to output: {:?}", error);
        std::process::exit(1);
    }

    let original_png_size = std::fs::read(input_path).unwrap().len();
    let size_difference: f64 = original_png_size as f64 - rugs_image_bytes.len() as f64;
    let percentage_difference = (size_difference / rugs_image_bytes.len() as f64) * 100.0;

    let elapsed_time = std::time::SystemTime::now().duration_since(start_time).expect("Time went backwards").as_secs_f32();
    println!("Done! Your RUGS image is {:.2}% ({:.2} KB) smaller than the inputted PNG", percentage_difference, (size_difference / 1000f64));
    println!("Entire operation took {:.2}s", elapsed_time);

    if !arguments_map.get("view").unwrap().is_none() {
        let image_bytes = new_rugs_image.image_bytes();
        let image = ImageView::new(ImageInfo::rgba8(width, height), &image_bytes);
        let window = create_window("Image", Default::default()).unwrap();
        window.set_image("Image", image).unwrap();
        
        tests::TimingDebugger::breakpoint();
    }
}
