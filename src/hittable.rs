use std::f64;

use crate::{material::Material, ray::Ray, vec3::Vec3};

pub struct HitRecord<'a> {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub material: &'a dyn Material,
}

pub trait Hittable {
    fn hit<'a>(&'a self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord<'a>>;
}

impl Hittable for Vec<Box<dyn Hittable + Send + Sync>> {
    fn hit<'a>(&'a self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord<'a>> {
        let mut closest: Option<HitRecord> = None;
        let mut closest_t = f64::INFINITY;

        for obj in self {
            if let Some(hit) = obj.hit(ray, t_min, t_max) {
                if hit.t < closest_t {
                    closest_t = hit.t;
                    closest = Some(hit);
                }
            }
        }

        closest
    }
}
