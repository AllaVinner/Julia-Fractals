use image;
use num_complex::Complex;
mod julia_fractal;
use clap::Parser;
use std::string::String;

type Real = julia_fractal::Real;

struct GridConfig {
    x_min: Real,
    x_max: Real,
    y_min: Real,
    y_max: Real,
}

struct ImageConfig {
    width: usize,
    height: usize
}

struct PixelCoordinate {
    width: usize,
    height: usize,    
}

struct GridCoordinate {
    x: Real,
    y: Real,
}

fn pixel_to_grid_coordinate(pixel: &PixelCoordinate, image_config: &ImageConfig, grid_config: &GridConfig) -> GridCoordinate {
    GridCoordinate { 
        x: (pixel.width as Real) * (grid_config.x_max - grid_config.x_min) / (image_config.width as Real) + grid_config.x_min,
        y: (pixel.height as Real) * (grid_config.y_min - grid_config.y_max)/ (image_config.height as Real) + grid_config.y_max,
    }
}

#[derive(Parser, Debug)]
#[command(author, version, 
    about="THis is a great command", 
    long_about = "Actually This is the greatest that ever existed")]
struct Args {

   #[arg(short, long, default_value_t = -0.5)]
   real: Real,
   
   #[arg(short, long, default_value_t = -0.5)]
   imag: Real,
  
   #[arg(long, default_value_t = -2.2)]
   real_min: Real,
  
   #[arg(long, default_value_t = 2.2)]
   real_max: Real,

   #[arg(long, default_value_t = -2.2)]
   imag_min: Real,
  
   #[arg(long, default_value_t = 2.2)]
   imag_max: Real,
  
   #[arg(long, default_value_t = 800)]
   image_height: usize,
  
   #[arg(long, default_value_t = 800)]
   image_width: usize,

   #[arg(short, long, default_value_t = String::from("fractal.png"))]
   save_path: String,

   #[arg(long, default_value_t = 40)]
   max_iterations: usize
}

fn main() {
    let args: Args = Args::parse();
    let grid_config = GridConfig { x_min: args.real_min, x_max: args.real_max, y_min: args.imag_min, y_max: args.imag_max};
    let image_config = ImageConfig { width: args.image_width, height: args.image_height};
    let constant_z = Complex::<Real>::new(args.real, args.imag);
    
    let max_iterations = args.max_iterations;

    let p = 2.;
    let iter_to_u8 = |i: usize| -> u8 {
        ((1.0 -(1.-(i as f32 / max_iterations as f32)).powf(p)).powf(1./p)*255.5) as u8
    };

    let img = image::ImageBuffer::from_fn(image_config.width as u32, image_config.height as u32, |w, h| {
        let grid_coordinate = pixel_to_grid_coordinate(&PixelCoordinate { width: w as usize, height: h as usize}, &image_config, &grid_config);
        let initial_z = Complex::<Real>::new( grid_coordinate.x, grid_coordinate.y);
        let value = julia_fractal::julia_iterations(initial_z, constant_z, max_iterations);
        let g = iter_to_u8(value);
        let r = (w as f32 / image_config.width as f32 * 255.5) as u8;
        let b = (h as f32 / image_config.height as f32 * 255.5) as u8;
        image::Rgb([r, g, b])
    });
    img.save(args.save_path).unwrap();
}

