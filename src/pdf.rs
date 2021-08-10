use std::f32::consts::{FRAC_1_PI, PI};

use rand::{prelude::ThreadRng, thread_rng};

use crate::{
    hittable::{HitModel, Hittable, HittableList},
    onb::ONB,
    utility::{random_double, random_in_hemisphere},
    vec3::{self, Point3, Vec3},
};

// Cosine-weighted hemisphere sampling using spherical coordinates.
#[inline]
pub fn random_cosine_direction(rng: &mut ThreadRng) -> Vec3 {
    let r1 = random_double(rng);
    let r2 = random_double(rng);
    let r2_sqrt = r2.sqrt();
    let z = (1.0 - r2).sqrt();
    let phi = 2.0 * PI * r1;

    let x = phi.cos() * r2_sqrt;
    let y = phi.sin() * r2_sqrt;

    Vec3::new(x, y, z)
}

pub enum PDF<'a> {
    Uniform(UniformPDF),
    Cosine(CosinePDF),
    Hittable(HittablePDF<'a>),
}

impl PDF<'_> {
    pub fn value(&self, direction: &Vec3) -> f32 {
        match self {
            Self::Uniform(p) => p.value(direction),
            Self::Cosine(p) => p.value(direction),
            Self::Hittable(p) => p.value(direction),
        }
    }

    pub fn generate(&self) -> Vec3 {
        match self {
            Self::Uniform(p) => p.generate(),
            Self::Cosine(p) => p.generate(),
            Self::Hittable(p) => p.generate(),
        }
    }
}

pub struct UniformPDF {
    normal: Vec3,
}

impl UniformPDF {
    pub fn new(n: &Vec3) -> Self {
        UniformPDF { normal: *n }
    }

    pub fn value(&self, direction: &Vec3) -> f32 {
        FRAC_1_PI / 2.0
    }

    pub fn generate(&self) -> Vec3 {
        random_in_hemisphere(&mut thread_rng(), &self.normal)
    }
}

pub struct CosinePDF {
    uvw: ONB,
}

impl CosinePDF {
    pub fn new(w: &Vec3) -> Self {
        CosinePDF { uvw: vec3::coordinate_system(w) }
    }

    pub fn value(&self, direction: &Vec3) -> f32 {
        let cosine = vec3::unit_vector(direction).dot(self.uvw.w());
        if cosine <= 0.0 {
            0.0
        } else {
            cosine / PI
        }
    }

    pub fn generate(&self) -> Vec3 {
        self.uvw.local_vec(&random_cosine_direction(&mut rand::thread_rng()))
    }
}

pub struct HittablePDF<'a> {
    origin: Point3,
    hittable: &'a HittableList<HitModel<'a>>,
}

impl<'a> HittablePDF<'a> {
    pub fn new(origin: &Point3, hittable: &'a HittableList<HitModel<'a>>) -> Self {
        HittablePDF { origin: *origin, hittable }
    }

    pub fn value(&self, direction: &Vec3) -> f32 {
        self.hittable.pdf_value(&self.origin, direction)
    }

    pub fn generate(&self) -> Vec3 {
        self.hittable.random(&self.origin)
    }
}

pub struct MixturePDF<'a> {
    p: [PDF<'a>; 2],
}

impl<'a> MixturePDF<'a> {
    pub fn new(p0: PDF<'a>, p1: PDF<'a>) -> Self {
        MixturePDF { p: [p0, p1] }
    }

    pub fn value(&self, direction: &Vec3) -> f32 {
        0.5 * self.p[0].value(direction) + 0.5 * self.p[1].value(direction)
    }

    pub fn generate(&self) -> Vec3 {
        let mut rng = rand::thread_rng();
        if random_double(&mut rng) < 0.5 {
            self.p[0].generate()
        } else {
            self.p[1].generate()
        }
    }
}
