use image::io::Reader as ImageReader;

use clap::{Parser, ValueEnum};
use std::path::PathBuf;
mod julia_fractal;

pub struct GridImageConfiguration {
    height: Pixels,
    width: Pixels,
    x_min: Number,
    x_max: Number,
    y_min: Number,
    y_max: Number,
}

impl Default for GridImageConfiguration {
    fn default() -> Self {
        Self { height: 800, width: 800, x_min: -3., x_max: 3., y_min: -3., y_max: 3.}
    }
}



#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   #[arg(short, long, default_value_t = -0.5)]
   real: Number,
   #[arg(short, long, default_value_t = 0.3)]
   imag: Number,
   #[arg(short, long, default_value_t = String::from("fractal.png"))]
   save_path: String,
   #[arg(long, default_value_t = 800)]
   image_height: Pixels,
   #[arg(long, default_value_t = 800)]
   image_width: Pixels,
   #[arg(long, default_value_t = -3.)]
   real_min: Number,
   #[arg(long, default_value_t = 3.)]
   real_max: Number,
   #[arg(long, default_value_t = -3.)]
   imag_min: Number,
   #[arg(long, default_value_t = 3.)]
   imag_max: Number,
}





fn main() {
    let args: Args = Args::parse();
    let image_config = GridImageConfiguration{
        height: args.image_height,
        width: args.image_width,
        x_min: args.real_min,
        x_max: args.real_max,
        y_min: args.imag_min,
        y_max: args.imag_max
    };

    julia_fractal::main(args.real, args.imag, image_config, PathBuf::from(args.save_path);
}

