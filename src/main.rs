mod hittable;
mod material;
mod ray;
mod sphere;
mod vec3;

use std::{f64, os::unix::fs::MetadataExt, sync::Arc};

use hittable::Hittable;
use rand::Rng;
use ray::Ray;
use rayon::prelude::*;
use sphere::Sphere;
use vec3::Vec3;

use crate::material::{Diffuse, Metal};

fn sky_colour(ray: Ray) -> Vec3 {
    let t = 0.5 * (ray.direction.unit_vector().y + 1.0);
    let light_blue = Vec3::new(0.5, 0.7, 1.);
    let dark_blue = Vec3::new(0.3, 0.6, 1.);
    (1. - t) * dark_blue + t * light_blue
}

fn ray_colour(ray: Ray, world: &dyn Hittable, depth: u32) -> Vec3 {
    if depth == 0 {
        return Vec3::new(0., 0., 0.);
    }

    if let Some(hit) = world.hit(ray, 0.001, f64::INFINITY) {
        let scatter = hit.material.scatter(ray, &hit);
        scatter.attenuation * ray_colour(scatter.ray, world, depth - 1)
    } else {
        sky_colour(ray)
    }
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width: u32 = 1920;
    let image_height = (image_width as f64 / aspect_ratio) as u32;

    println!("P3\n{image_width} {image_height}\n255");

    let viewport_height = 2.0;
    let viewport_width = viewport_height * aspect_ratio;
    let focal_length = 1.;
    let camera_origin = Vec3::new(0., 0., 0.);
    let samples_per_pixel = 64;

    let port_image_hratio = viewport_height / image_height as f64;
    let port_image_wratio = viewport_width / image_width as f64;

    let top_left =
        camera_origin + Vec3::new(-viewport_width / 2., viewport_height / 2., -focal_length);
    let objects: Arc<Vec<Box<dyn Hittable + Send + Sync>>> = Arc::new(vec![
        Box::new(Sphere::new(
            Vec3::new(0., 0.3, -1.),
            0.5,
            Metal::new(Vec3::new(0.8, 0.8, 0.8)),
        )),
        Box::new(Sphere::new(
            Vec3::new(0., -10000.5, 0.),
            10000.,
            Diffuse::new(Vec3::new(0.3, 0., 0.)),
        )),
    ]);

    let pixels: Vec<Vec3> = (0..image_height)
        .into_par_iter()
        .flat_map(|y| {
            let objects = Arc::clone(&objects);
            (0..image_width)
                .map(move |x| {
                    let mut rng = rand::rng();

                    let mut colour = Vec3::new(0., 0., 0.);

                    for _ in 0..samples_per_pixel {
                        let u_offset: f64 = rng.random_range(0.0..1.0);
                        let v_offset: f64 = rng.random_range(0.0..1.0);

                        let ray = Ray::new(
                            camera_origin,
                            top_left + Vec3::new((x as f64 + u_offset) * port_image_wratio, 0., 0.)
                                - Vec3::new(0., (y as f64 + v_offset) * port_image_hratio, 0.),
                        );

                        colour = colour + ray_colour(ray, &*objects, 50);
                    }

                    colour / samples_per_pixel as f64
                })
                .collect::<Vec<_>>()
        })
        .collect();

    // Then print them
    for colour in pixels {
        println!(
            "{} {} {}",
            (colour.x.clamp(0., 1.).sqrt() * 255.99) as u8,
            (colour.y.clamp(0., 1.).sqrt() * 255.99) as u8,
            (colour.z.clamp(0., 1.).sqrt() * 255.99) as u8
        );
    }
}
