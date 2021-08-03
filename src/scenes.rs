use std::sync::Arc;

use image::{ImageBuffer, Rgb};

use crate::bvh::BVHNode;
use crate::camera::Camera;
use crate::hittable::HitModel;
use crate::instances::{RotateY, Translate};
use crate::perlin::Perlin;
use crate::rect::{Box, XYRect, XZRect, YZRect};
use crate::sphere::Sphere;
use crate::vec3::Color;
use crate::volumes::Constant;
use crate::{color, utility::*, ASPECT_RATIO};
use crate::{
    hittable::HittableList, material::Surface, sphere::MovingSphere, texture::SurfaceTexture,
    vec3::Vec3,
};

pub fn random_scene<'a>() -> (HittableList<MovingSphere<'a>>, Camera, Color) {
    let mut world = HittableList::new();

    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let vfov = 20.0;
    let background = Vec3::new(0.7, 0.8, 1.0);

    let camera =
        Camera::new(lookfrom, lookat, vup, vfov, ASPECT_RATIO, aperture, dist_to_focus, 0.0, 1.0);

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

    (world, camera, background)
}

pub fn two_spheres<'a>() -> (HittableList<Sphere<'a>>, Camera, Color) {
    let mut objects = HittableList::new();

    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let vfov = 20.0;
    let background = Vec3::new(0.7, 0.8, 1.0);

    let camera =
        Camera::new(lookfrom, lookat, vup, vfov, ASPECT_RATIO, aperture, dist_to_focus, 0.0, 0.0);
    let checkered = SurfaceTexture::Checkered(Vec3::new(0.2, 0.3, 0.1), Vec3::new(0.9, 0.9, 0.9));

    objects.add(Sphere::new(Vec3::new(0.0, -10.0, 0.0), 10.0, Surface::Lambertian(checkered)));
    objects.add(Sphere::new(Vec3::new(0.0, 10.0, 0.0), 10.0, Surface::Lambertian(checkered)));

    (objects, camera, background)
}

pub fn two_perlin_spheres<'a>() -> (HittableList<Sphere<'a>>, Camera, Color) {
    let mut objects = HittableList::new();

    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let vfov = 20.0;
    let background = Vec3::new(0.7, 0.8, 1.0);

    let camera =
        Camera::new(lookfrom, lookat, vup, vfov, ASPECT_RATIO, aperture, dist_to_focus, 0.0, 0.0);

    let perlin = SurfaceTexture::Noise(Perlin::new(), 4.0);
    objects.add(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Surface::Lambertian(perlin)));
    objects.add(Sphere::new(Vec3::new(0.0, 2.0, 0.0), 2.0, Surface::Lambertian(perlin)));

    (objects, camera, background)
}

pub fn earth<'a>(
    buffer: &'a ImageBuffer<Rgb<u8>, Vec<u8>>,
) -> (HittableList<Sphere<'a>>, Camera, Color) {
    let mut objects = HittableList::new();

    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let vfov = 20.0;
    let background = Vec3::new(0.7, 0.8, 1.0);

    let camera =
        Camera::new(lookfrom, lookat, vup, vfov, ASPECT_RATIO, aperture, dist_to_focus, 0.0, 0.0);
    let earth = SurfaceTexture::Image { buffer, width: buffer.width(), height: buffer.height() };
    let earth_surface = Surface::Lambertian(earth);

    objects.add(Sphere::new(Vec3::new(0.0, 0.0, 0.0), 2.0, earth_surface));

    (objects, camera, background)
}

pub fn simple_light<'a>() -> (HittableList<HitModel<'a>>, Camera, Color) {
    let mut objects = HittableList::new();

    let lookfrom = Vec3::new(26.0, 3.0, 6.0);
    let lookat = Vec3::new(0.0, 2.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let vfov = 20.0;
    let background = Vec3::new(0.0, 0.0, 0.0);

    let camera =
        Camera::new(lookfrom, lookat, vup, vfov, ASPECT_RATIO, aperture, dist_to_focus, 0.0, 0.0);

    let perlin = SurfaceTexture::Noise(Perlin::new(), 4.0);
    let perlin_surface = Surface::Lambertian(perlin);
    objects.add(HitModel::Sphere(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        perlin_surface,
    )));
    objects.add(HitModel::Sphere(Sphere::new(Vec3::new(0.0, 2.0, 0.0), 2.0, perlin_surface)));

    // Values greater than Vec3(1.0, 1.0, 1.0) allow for emission of light
    let light = Surface::DiffuseLight(SurfaceTexture::Solid(Vec3::new(4.0, 4.0, 4.0)));
    objects.add(HitModel::XYRect(XYRect::new(3.0, 5.0, 1.0, 3.0, -2.0, light)));

    (objects, camera, background)
}

pub fn cornell_box<'a>() -> (HittableList<HitModel<'a>>, Camera, Color) {
    let mut objects = HittableList::new();

    let lookfrom = Vec3::new(278.0, 278.0, -800.0);
    let lookat = Vec3::new(278.0, 278.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let vfov = 40.0;
    let background = Vec3::new(0.0, 0.0, 0.0);

    let camera = Camera::new(lookfrom, lookat, vup, vfov, 1.0, aperture, dist_to_focus, 0.0, 0.0);

    let red = Surface::Lambertian(SurfaceTexture::Solid(Vec3::new(0.65, 0.05, 0.05)));
    let white = Surface::Lambertian(SurfaceTexture::Solid(Vec3::new(0.73, 0.73, 0.73)));
    let green = Surface::Lambertian(SurfaceTexture::Solid(Vec3::new(0.12, 0.45, 0.15)));
    let light = Surface::DiffuseLight(SurfaceTexture::Solid(Vec3::new(15.0, 15.0, 15.0)));

    objects.add(HitModel::YZRect(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    objects.add(HitModel::YZRect(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    objects.add(HitModel::XZRect(XZRect::new(213.0, 343.0, 227.0, 332.0, 554.0, light)));
    objects.add(HitModel::XZRect(XZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, white)));
    objects.add(HitModel::XZRect(XZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white)));
    objects.add(HitModel::XYRect(XYRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white)));

    let box1 =
        HitModel::Box(Box::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(165.0, 330.0, 165.0), white));
    let box1 = HitModel::RotateY(RotateY::new(box1, 15.0));
    let box1 = HitModel::Translate(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
    objects.add(box1);

    let box2 =
        HitModel::Box(Box::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(165.0, 165.0, 165.0), white));
    let box2 = HitModel::RotateY(RotateY::new(box2, -18.0));
    let box2 = HitModel::Translate(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));
    objects.add(box2);

    (objects, camera, background)
}

pub fn cornell_smoke<'a>() -> (HittableList<HitModel<'a>>, Camera, Color) {
    let mut objects = HittableList::new();

    let lookfrom = Vec3::new(278.0, 278.0, -800.0);
    let lookat = Vec3::new(278.0, 278.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let vfov = 40.0;
    let background = Vec3::new(0.0, 0.0, 0.0);

    let camera = Camera::new(lookfrom, lookat, vup, vfov, 1.0, aperture, dist_to_focus, 0.0, 0.0);

    let red = Surface::Lambertian(SurfaceTexture::Solid(Vec3::new(0.65, 0.05, 0.05)));
    let white = Surface::Lambertian(SurfaceTexture::Solid(Vec3::new(0.73, 0.73, 0.73)));
    let green = Surface::Lambertian(SurfaceTexture::Solid(Vec3::new(0.12, 0.45, 0.15)));
    let light = Surface::DiffuseLight(SurfaceTexture::Solid(Vec3::new(15.0, 15.0, 15.0)));

    objects.add(HitModel::YZRect(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    objects.add(HitModel::YZRect(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    objects.add(HitModel::XZRect(XZRect::new(213.0, 343.0, 227.0, 332.0, 554.0, light)));
    objects.add(HitModel::XZRect(XZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, white)));
    objects.add(HitModel::XZRect(XZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white)));
    objects.add(HitModel::XYRect(XYRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white)));

    let box1 =
        HitModel::Box(Box::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(165.0, 330.0, 165.0), white));
    let box1 = HitModel::RotateY(RotateY::new(box1, 15.0));
    let box1 = HitModel::Translate(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));

    let box2 =
        HitModel::Box(Box::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(165.0, 165.0, 165.0), white));
    let box2 = HitModel::RotateY(RotateY::new(box2, -18.0));
    let box2 = HitModel::Translate(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));

    objects.add(HitModel::Constant(Constant::new(0.01, box1, Vec3::new(0.0, 0.0, 0.0))));
    objects.add(HitModel::Constant(Constant::new(0.01, box2, Vec3::new(1.0, 1.0, 1.0))));

    (objects, camera, background)
}

pub fn final_scene<'a>() -> (HittableList<HitModel<'a>>, Camera, Color) {
    let mut objects = HittableList::new();
    let mut boxes1 = Vec::new();

    let lookfrom = Vec3::new(478.0, 278.0, -600.0);
    let lookat = Vec3::new(278.0, 278.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let vfov = 40.0;
    let background = Vec3::new(0.0, 0.0, 0.0);

    let camera = Camera::new(lookfrom, lookat, vup, vfov, 1.0, aperture, dist_to_focus, 0.0, 0.0);

    let mut rng = rand::thread_rng();
    let ground_material = Surface::Lambertian(SurfaceTexture::Solid(Vec3::new(0.48, 0.83, 0.53)));

    for i in 0..20 {
        for j in 0..20 {
            let w = 100.0;
            let x0 = -1000.0 + (i as f32) * w;
            let z0 = -1000.0 + (j as f32) * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_double_range(&mut rng, 1.0, 101.0);
            let z1 = z0 + w;

            boxes1.push(Arc::new(HitModel::Box(Box::new(
                Vec3::new(x0, y0, z0),
                Vec3::new(x1, y1, z1),
                ground_material,
            ))));
        }
    }

    let boxes1_len = boxes1.len();
    objects.add(HitModel::BVH(BVHNode::new(&mut boxes1, 0, boxes1_len, 0, 1)));

    let light = Surface::DiffuseLight(SurfaceTexture::Solid(Vec3::new(7.0, 7.0, 7.0)));
    objects.add(HitModel::XZRect(XZRect::new(123.0, 423.0, 147.0, 412.0, 553.0, light)));

    let center1 = Vec3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let moving_sphere = HitModel::MovingSphere(MovingSphere::new(
        center1,
        center2,
        0.0,
        1.0,
        50.0,
        Surface::Lambertian(SurfaceTexture::Solid(Vec3::new(0.7, 0.3, 0.1))),
    ));
    objects.add(moving_sphere);

    objects.add(HitModel::Sphere(Sphere::new(
        Vec3::new(260.0, 150.0, 45.0),
        50.0,
        Surface::Dielectric(1.5),
    )));
    objects.add(HitModel::Sphere(Sphere::new(
        Vec3::new(0.0, 150.0, 145.0),
        50.0,
        Surface::Metal(Vec3::new(0.8, 0.8, 0.9), 1.0),
    )));

    let boundary = HitModel::Sphere(Sphere::new(
        Vec3::new(360.0, 150.0, 145.0),
        70.0,
        Surface::Dielectric(1.5),
    ));
    objects.add(boundary.clone());
    objects.add(HitModel::Constant(Constant::new(0.2, boundary, Vec3::new(0.2, 0.4, 0.9))));

    let boundary =
        HitModel::Sphere(Sphere::new(Vec3::new(0.0, 0.0, 0.0), 5000.0, Surface::Dielectric(1.5)));
    objects.add(HitModel::Constant(Constant::new(0.0001, boundary, Vec3::new(1.0, 1.0, 1.0))));

    let perlin = SurfaceTexture::Noise(Perlin::new(), 0.1);
    objects.add(HitModel::Sphere(Sphere::new(
        Vec3::new(220.0, 280.0, 300.0),
        80.0,
        Surface::Lambertian(perlin),
    )));

    let mut boxes2 = Vec::new();
    let white = Surface::Lambertian(SurfaceTexture::Solid(Vec3::new(0.73, 0.73, 0.73)));
    for i in 0..1000 {
        boxes2.push(Arc::new(HitModel::Sphere(Sphere::new(
            random_vec_range(&mut rng, 0.0, 165.0),
            10.0,
            white,
        ))));
    }

    let boxes2_len = boxes2.len();
    objects.add(HitModel::Translate(Translate::new(
        HitModel::RotateY(RotateY::new(
            HitModel::BVH(BVHNode::new(&mut boxes2, 0, boxes2_len, 0, 1)),
            15.0,
        )),
        Vec3::new(-100.0, 270.0, 395.0),
    )));

    (objects, camera, background)
}

pub fn random_bvh<'a>() -> (HittableList<HitModel<'a>>, Camera, Color) {
    let mut world = Vec::new();

    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let vfov = 20.0;
    let background = Vec3::new(0.7, 0.8, 1.0);

    let camera =
        Camera::new(lookfrom, lookat, vup, vfov, ASPECT_RATIO, aperture, dist_to_focus, 0.0, 1.0);

    let checkered = SurfaceTexture::Checkered(Vec3::new(0.2, 0.3, 0.1), Vec3::new(0.9, 0.9, 0.9));
    let ground_material = Surface::Lambertian(checkered);

    let material1 = Surface::Dielectric(1.5);
    let material2 = Surface::Lambertian(SurfaceTexture::Solid(Vec3::new(0.4, 0.2, 0.1)));
    let material3 = Surface::Metal(Vec3::new(0.7, 0.6, 0.5), 0.0);

    world.push(Arc::new(HitModel::MovingSphere(MovingSphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        Vec3::new(0.0, -1000.0, 0.0),
        0.0,
        1.0,
        1000.0,
        ground_material,
    ))));

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
                    world.push(Arc::new(HitModel::MovingSphere(MovingSphere::new(
                        center,
                        center2,
                        0.0,
                        1.0,
                        0.2,
                        sphere_material,
                    ))));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = color::random_range(0.5, 1.0);
                    let fuzz = random_double_range(&mut rng, 0.0, 0.5);
                    let sphere_material = Surface::Metal(albedo, fuzz);
                    world.push(Arc::new(HitModel::MovingSphere(MovingSphere::new(
                        center,
                        center,
                        0.0,
                        1.0,
                        0.2,
                        sphere_material,
                    ))));
                } else {
                    // glass
                    let sphere_material = Surface::Dielectric(1.5);
                    world.push(Arc::new(HitModel::MovingSphere(MovingSphere::new(
                        center,
                        center,
                        0.0,
                        1.0,
                        0.2,
                        sphere_material,
                    ))));
                }
            }
        }
    }

    world.push(Arc::new(HitModel::MovingSphere(MovingSphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        0.0,
        1.0,
        1.0,
        material1,
    ))));
    world.push(Arc::new(HitModel::MovingSphere(MovingSphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        Vec3::new(-4.0, 1.0, 0.0),
        0.0,
        1.0,
        1.0,
        material2,
    ))));
    world.push(Arc::new(HitModel::MovingSphere(MovingSphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        Vec3::new(4.0, 1.0, 0.0),
        0.0,
        1.0,
        1.0,
        material3,
    ))));

    let mut objects = HittableList::new();
    let world_len = world.len();
    objects.add(HitModel::BVH(BVHNode::new(&mut world, 0, world_len, 0, 1)));

    (objects, camera, background)
}
