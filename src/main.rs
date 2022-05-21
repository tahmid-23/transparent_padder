use std::path::PathBuf;

use clap::Parser;
use image::{GenericImage, ImageBuffer};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(parse(from_os_str))]
    input: PathBuf,
}

fn main() {
    let args = Args::parse();
    let input_image = image::open(&args.input)
        .expect("Failed to parse image!");

    let image_width = input_image.width();
    let image_height = input_image.height();

    let diameter_squared = image_width * image_width + image_height * image_height;
    let diameter = (diameter_squared as f32).sqrt();

    let delta_width = ((diameter - image_width as f32) / 2f32).ceil() as u32;
    let delta_height = ((diameter - image_height as f32) / 2f32).ceil() as u32;

    let new_width = image_width + 2 * delta_width;
    let new_height = image_height + 2 * delta_height;

    let mut output_image = ImageBuffer::new(new_width, new_height);
    output_image.copy_from(&input_image, delta_width, delta_height)
        .expect("Could not copy image!");

    output_image.save("result.png").expect("Could not save image!");
}
