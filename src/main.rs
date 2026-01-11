mod camera;
mod hittable;
mod material;
mod ray;
mod sphere;
mod vec3;

use hittable::Hittable;
use indicatif::{ParallelProgressIterator, ProgressBar, ProgressStyle};
use rand::{Rng, SeedableRng, rngs::SmallRng};
use ray::Ray;
use rayon::prelude::*;
use sphere::Sphere;
use vec3::Vec3;

use material::{Diffuse, Light, Reflective};
use std::{f64, sync::Arc};

use crate::{camera::Camera, material::Dielectric};

fn sky_colour(ray: Ray) -> Vec3 {
    let t = 0.5 * (ray.direction.unit_vector().y + 1.0);
    let light_blue = Vec3::new(0.2, 0.5, 1.);
    let dark_blue = Vec3::new(0.1, 0.3, 0.9);
    (1. - t) * dark_blue + t * light_blue
}

fn ray_colour(ray: Ray, world: &dyn Hittable, depth: u32, rng: &mut SmallRng) -> Vec3 {
    if depth == 0 {
        return Vec3::new(0., 0., 0.);
    }

    if let Some(hit) = world.hit(ray, 0.001, f64::INFINITY) {
        let emitted = hit.material.emmited();

        if let Some(scatter) = hit.material.scatter(ray, &hit, rng) {
            emitted + scatter.attenuation * ray_colour(scatter.ray, world, depth - 1, rng)
        } else {
            emitted
        }
    } else {
        sky_colour(ray)
    }
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width: u32 = 1920;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 4096;

    println!("P3\n{image_width} {image_height}\n255");

    let cam = Camera::new(
        Vec3::new(6., 4., 0.),
        Vec3::new(0., 0., -25.),
        Vec3::new(0., 1., 0.),
        50.,
        aspect_ratio,
    );

    let objects: Arc<Vec<Box<dyn Hittable + Send + Sync>>> = Arc::new(vec![
        Box::new(Sphere::new(
            Vec3::new(0., -10000.5, 0.),
            10000.,
            Diffuse::new(Vec3::new(1., 1., 1.)),
        )),
        Box::new(Sphere::new(
            Vec3::new(0., 1.5, -15.),
            2.,
            Dielectric::new(Vec3::new(0.5, 1., 0.5), 1.5),
        )),
        Box::new(Sphere::new(
            Vec3::new(4., 1.5, -14.),
            0.5,
            Light::new(Vec3::new(1., 0.87, 0.13), 100.),
        )),
        Box::new(Sphere::new(
            Vec3::new(-12., 8.5, -35.),
            9.,
            Reflective::new(Vec3::new(0.9, 0., 0.), 0.),
        )),
        Box::new(Sphere::new(
            Vec3::new(8., 3.5, -30.),
            4.,
            Reflective::new(Vec3::new(0.7, 0.7, 0.7), 0.),
        )),
    ]);

    let progress_bar = ProgressBar::new(image_height as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}|{eta_precise}] {wide_bar:.cyan/blue} {pos:>4}/{len:4}")
            .unwrap(),
    );

    let pixels: Vec<Vec3> = (0..image_height)
        .into_par_iter()
        .progress_with(progress_bar)
        .flat_map(|y| {
            let objects = Arc::clone(&objects);
            let mut rng = rand::rngs::SmallRng::from_os_rng();
            (0..image_width)
                .map(move |x| {
                    let mut colour = Vec3::new(0., 0., 0.);

                    for _ in 0..samples_per_pixel {
                        let u_offset: f64 = rng.random_range(0.0..1.0);
                        let v_offset: f64 = rng.random_range(0.0..1.0);

                        let ray = Ray::new(
                            cam.origin,
                            cam.top_left
                                + cam.horizontal * ((x as f64 + u_offset) / image_width as f64)
                                - cam.vertical * ((y as f64 + v_offset) / image_height as f64)
                                - cam.origin,
                        );

                        colour = colour + ray_colour(ray, &*objects, 10000, &mut rng);
                    }

                    colour / samples_per_pixel as f64
                })
                .collect::<Vec<_>>()
        })
        .collect();

    for colour in pixels {
        println!(
            "{} {} {}",
            (colour.x.clamp(0., 1.).sqrt() * 255.99) as u8,
            (colour.y.clamp(0., 1.).sqrt() * 255.99) as u8,
            (colour.z.clamp(0., 1.).sqrt() * 255.99) as u8
        );
    }
}
