use crate::vec::Axis::*;
use crate::vec::Vec3;

use rand::Rng;

fn perlin_generate() -> Vec<f64> {
    let mut rng = rand::thread_rng();
    let mut p = Vec::with_capacity(256);

    for _ in 0..256 {
        p.push(rng.gen::<f64>());
    }
    p
}

fn permute(p: &mut [usize], n: usize) {
    let mut rng = rand::thread_rng();
    for i in (0..n).rev() {
        let target = rng.gen_range(0, i + 1);
        p.swap(i, target);
    }
}

fn perlin_generate_perm() -> Vec<usize> {
    let mut p = Vec::with_capacity(256);
    for i in 0..256 {
        p.push(i);
    }
    permute(&mut p, 256);
    p
}

#[derive(Clone)]
pub struct Perlin {
    ran_float: Vec<f64>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Perlin {
    pub fn new() -> Self {
        Self {
            ran_float: perlin_generate(),
            perm_x: perlin_generate_perm(),
            perm_y: perlin_generate_perm(),
            perm_z: perlin_generate_perm(),
        }
    }

    pub fn noise(&self, p: Vec3) -> f64 {
        let i = (4.0 * p[X]).abs() as usize & 7;
        let j = (4.0 * p[Y]).abs() as usize & 7;
        let k = (4.0 * p[Z]).abs() as usize & 7;
        self.ran_float[self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]]
    }
}
