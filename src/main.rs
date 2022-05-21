use std::path::PathBuf;

use clap::Parser;
use image::{GenericImage, ImageBuffer};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(parse(from_os_str))]
    input: PathBuf,

    #[clap(short, long, default_value_t = 250)]
    width_padding: u32,

    #[clap(short, long, default_value_t = 250)]
    height_padding: u32
}

fn main() {
    let args = Args::parse();
    let input_image = image::open(&args.input)
        .expect("Failed to parse image!");

    let width_padding = args.width_padding;
    let height_padding = args.height_padding;
    let new_width = input_image.width() + 2 * width_padding;
    let new_height = input_image.height() + 2 * height_padding;

    let mut output_image = ImageBuffer::new(new_width, new_height);
    output_image.copy_from(&input_image, width_padding, height_padding)
        .expect("Could not copy image!");

    output_image.save("result.png").expect("Could not save image!");
}
