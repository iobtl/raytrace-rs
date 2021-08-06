use std::ops::Index;

use crate::vec3::{self, Vec3};

pub struct ONB {
    uvw: [Vec3; 3],
}

impl ONB {
    pub fn new(w: &Vec3) -> Self {
        let w = vec3::unit_vector(w);
        let a = if w.x() > 0.9 { Vec3::new(0.0, 1.0, 0.0) } else { Vec3::new(1.0, 0.0, 0.0) };
        let v = vec3::unit_vector(&w.cross(&a));
        let u = w.cross(&v);

        ONB { uvw: [u, v, w] }
    }

    pub fn u(&self) -> &Vec3 {
        &self.uvw[0]
    }

    pub fn v(&self) -> &Vec3 {
        &self.uvw[1]
    }

    pub fn w(&self) -> &Vec3 {
        &self.uvw[2]
    }

    pub fn local(&self, a: f32, b: f32, c: f32) -> Vec3 {
        *self.u() * a + *self.v() * b + *self.w() * c
    }

    pub fn local_vec(&self, v: &Vec3) -> Vec3 {
        *self.u() * v.x() + *self.v() * v.y() + *self.w() * v.z()
    }
}

impl Index<usize> for ONB {
    type Output = Vec3;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.uvw[0],
            1 => &self.uvw[1],
            2 => &self.uvw[2],
            _ => panic!("Invalid indexing into coordinate system"),
        }
    }
}
