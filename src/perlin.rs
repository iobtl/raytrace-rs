use crate::{
    utility::{random_int_range, random_vec_range},
    vec3::{self, Point3, Vec3},
};

const POINT_COUNT: usize = 256;
const NOISE_DEPTH: i32 = 7;

#[derive(Copy, Clone)]
pub struct Perlin {
    ranvec: [Vec3; POINT_COUNT],
    perm_x: [i32; POINT_COUNT],
    perm_y: [i32; POINT_COUNT],
    perm_z: [i32; POINT_COUNT],
}

impl Perlin {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut ranvec: [Vec3; POINT_COUNT] = [Vec3::new(0.0, 0.0, 0.0); POINT_COUNT];
        for i in 0..POINT_COUNT {
            ranvec[i] = vec3::unit_vector(&random_vec_range(&mut rng, -1.0, 1.0));
        }

        let perm_x = perlin_generate_perm();
        let perm_y = perlin_generate_perm();
        let perm_z = perlin_generate_perm();

        Perlin { ranvec, perm_x, perm_y, perm_z }
    }

    pub fn turb(&self, p: &Point3) -> f32 {
        let mut accum = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;

        for i in 0..NOISE_DEPTH {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        accum.abs()
    }

    fn noise(&self, p: &Point3) -> f32 {
        // Random hashing to generate blocks of 'noise'
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();

        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;

        let mut c: [[[Vec3; 2]; 2]; 2] = [[[Vec3::new(0.0, 0.0, 0.0); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.ranvec[(self.perm_x[((i + di as i32) & 255) as usize]
                        ^ self.perm_y[((j + dj as i32) & 255) as usize]
                        ^ self.perm_z[((k + dk as i32) & 255) as usize])
                        as usize];
                }
            }
        }

        perlin_interp(c, u, v, w)
    }
}

fn perlin_generate_perm() -> [i32; POINT_COUNT] {
    let mut p: [i32; POINT_COUNT] = [0; POINT_COUNT];
    for i in 0..POINT_COUNT {
        p[i] = i as i32;
    }

    permute(&mut p, POINT_COUNT);

    p
}

fn permute(p: &mut [i32; POINT_COUNT], n: usize) {
    let mut rng = rand::thread_rng();
    for i in (1..n).into_iter().rev() {
        let target = random_int_range(&mut rng, 0, i as i32) as usize;
        let tmp = p[i];
        p[i] = p[target];
        p[target] = tmp;
    }
}

fn perlin_interp(c: [[[Vec3; 2]; 2]; 2], u: f32, v: f32, w: f32) -> f32 {
    let u = u * u * (3.0 - 2.0 * u);
    let v = v * v * (3.0 - 2.0 * v);
    let w = w * w * (3.0 - 2.0 * w);
    let mut accum = 0.0;

    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let i = i as f32;
                let j = j as f32;
                let k = k as f32;

                let weight_v = Vec3::new(u - i, v - j, w - k);
                accum += (i * u + (1.0 - i) * (1.0 - u))
                    * (j * v + (1.0 - j) * (1.0 - v))
                    * (k * w + (1.0 - k) * (1.0 - w))
                    * c[i as usize][j as usize][k as usize].dot(&weight_v);
            }
        }
    }

    accum
}
