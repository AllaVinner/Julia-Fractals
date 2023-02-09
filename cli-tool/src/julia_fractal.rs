use num_complex::Complex;

pub type Real = f32;

pub fn julia_iterations(initial_z: Complex<Real>, constant_z: Complex<Real>, max_iterations: usize) -> usize {
    let mut i: usize = 0;
    let mut z = initial_z;
    let c = constant_z;

    while i < max_iterations && z.norm() <= 2.0 {
        z = z * z + c;
        i += 1;
    }
    i
}
