use crate::sphere::Sphere;
use crate::{color, utility::*};
use crate::{
    hittable::HittableList, material::Surface, sphere::MovingSphere, texture::SurfaceTexture,
    vec3::Vec3,
};

pub fn random_scene() -> HittableList<MovingSphere> {
    let mut world = HittableList::new();

    let checkered = SurfaceTexture::Checkered(Vec3::new(0.2, 0.3, 0.1), Vec3::new(0.9, 0.9, 0.9));
    let ground_material = Surface::Lambertian(checkered);

    let material1 = Surface::Dielectric(1.5);
    let material2 = Surface::Lambertian(SurfaceTexture::Solid(Vec3::new(0.4, 0.2, 0.1)));
    let material3 = Surface::Metal(Vec3::new(0.7, 0.6, 0.5), 0.0);

    world.add(MovingSphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        Vec3::new(0.0, -1000.0, 0.0),
        0.0,
        1.0,
        1000.0,
        ground_material,
    ));

    let mut rng = rand::thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let a = a as f32;
            let b = b as f32;

            let choose_mat = random_double(&mut rng);
            let center = Vec3::new(
                a + 0.9 * random_double(&mut rng),
                0.2,
                b + 0.9 * random_double(&mut rng),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = color::random() * color::random();
                    let sphere_material = Surface::Lambertian(SurfaceTexture::Solid(albedo));
                    let center2 =
                        center + Vec3::new(0.0, random_double_range(&mut rng, 0.0, 0.5), 0.0);
                    world.add(MovingSphere::new(center, center2, 0.0, 1.0, 0.2, sphere_material));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = color::random_range(0.5, 1.0);
                    let fuzz = random_double_range(&mut rng, 0.0, 0.5);
                    let sphere_material = Surface::Metal(albedo, fuzz);
                    world.add(MovingSphere::new(center, center, 0.0, 1.0, 0.2, sphere_material));
                } else {
                    // glass
                    let sphere_material = Surface::Dielectric(1.5);
                    world.add(MovingSphere::new(center, center, 0.0, 1.0, 0.2, sphere_material));
                }
            }
        }
    }

    world.add(MovingSphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        0.0,
        1.0,
        1.0,
        material1,
    ));
    world.add(MovingSphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        Vec3::new(-4.0, 1.0, 0.0),
        0.0,
        1.0,
        1.0,
        material2,
    ));
    world.add(MovingSphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        Vec3::new(4.0, 1.0, 0.0),
        0.0,
        1.0,
        1.0,
        material3,
    ));

    world
}

pub fn two_spheres() -> HittableList<Sphere> {
    let mut objects = HittableList::new();
    let checkered = SurfaceTexture::Checkered(Vec3::new(0.2, 0.3, 0.1), Vec3::new(0.9, 0.9, 0.9));

    objects.add(Sphere::new(Vec3::new(0.0, -10.0, 0.0), 10.0, Surface::Lambertian(checkered)));
    objects.add(Sphere::new(Vec3::new(0.0, 10.0, 0.0), 10.0, Surface::Lambertian(checkered)));

    objects
}
