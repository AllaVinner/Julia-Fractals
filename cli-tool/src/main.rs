use image;
use num_complex::Complex;

type Real = f32;


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
        x: pixel.width as Real * (grid_config.x_max - grid_config.x_min)/ image_config.width as Real + grid_config.x_min,
        y: pixel.height as Real * (grid_config.y_min - grid_config.y_max)/ image_config.height as Real + grid_config.y_max,
    }
}

fn julia_iterations(initial_z: Complex<Real>, constant_z: Complex<Real>, max_iterations: usize) -> usize {
    let mut i: usize = 0;
    let mut z = initial_z;
    let c = constant_z;

    while i < max_iterations && z.norm() <= 2.0 {
        z = z * z + c;
        i += 1;
    }
    i
}

fn main() {
    let width = 400;
    let heigh = 600;
    let grid_config = GridConfig { x_min: -3., x_max: 3., y_min: -3.0, y_max: 3.};
    let image_config = ImageConfig { width: 800, height: 800 };
    let constant_z = Complex::<Real>::new(-0.23, 0.5);

    let img = image::ImageBuffer::from_fn(512, 512, |w, h| {
        let grid_coordinate = pixel_to_grid_coordinate(&PixelCoordinate { width: w as usize, height: h as usize}, &image_config, &grid_config);
        let initial_z = Complex::<Real>::new(grid_coordinate.x, grid_coordinate.y);
        let value = julia_iterations(initial_z, constant_z, 20);
        image::Rgb([0, 0, 10*value as u8])
    });
    img.save("pic.png").unwrap();
}

