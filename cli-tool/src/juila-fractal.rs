use image::io::Reader as ImageReader;
use image;
use num_complex;
use core::panic;
use std::env;
use clap::{Parser, ValueEnum};
use std::path::PathBuf;

type Pixels = u32;
type Number = f64;


pub fn main() {
    let args: Args = Args::parse();
    let image_config = GridImageConfiguration{
        height: args.image_height,
        width: args.image_width,
        x_min: args.real_min,
        x_max: args.real_max,
        y_min: args.imag_min,
        y_max: args.imag_max
    };

    fractal(args.real, args.imag, args.fractal, image_config, PathBuf::from(args.save_path));
}

fn julia_iterations(initial_z: Complex, constant_z: Complex, max_iterations: usize) -> usize {
    let mut i: usize = 0;
    let mut z = initial_z;
    let c = constant_z;

    while i < max_iterations && z.norm() <= 2.0 {
        z = z * z + c;
        i += 1;
    }
    i
}



fn julia_fractal(c_re: Number, c_im: Number, config: GridImageConfiguration, save_path: PathBuf) {
    let c = num_complex::Complex::new(c_re, c_im);

    let range_x = config.x_max-config.x_min;
    let range_y = config.y_max-config.y_min;

    let scalex =  range_x/ config.width as Number;
    let scaley = range_y/ config.height as Number;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(config.width, config.height);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (255.5/config.width as f32 * x as f32) as u8;
        let b = (255.5/config.height as f32 * y as f32) as u8;
        *pixel = image::Rgb([r, 0, b]);
    }

    // A redundant loop to demonstrate reading image data
    for pixel_x in 0..config.width {
        for pixel_y in 0..config.height {
            let x = pixel_x as Number * scalex - range_x/2.;
            let y = -(pixel_y as Number * scaley - range_y/2.);

            
            let mut z = num_complex::Complex::new(x, y);

            let mut i = 0;
            let factor: u32 = 20;
            while i < 255/factor && z.norm() <= 2.0 {
                z = z * z + c;
                i += 1;
            }

            let pixel = imgbuf.get_pixel_mut(pixel_x, pixel_y);
            let image::Rgb(data) = *pixel;
            *pixel = image::Rgb([data[0], (i*factor) as u8, data[2]]);
        }
    }

    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save(save_path) .unwrap();
}


