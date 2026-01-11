use crate::{
    hittable::{HitRecord, Hittable},
    material::Material,
    ray::Ray,
    vec3::Vec3,
};

pub struct Sphere<M: Material> {
    pub center: Vec3,
    pub radius: f64,
    pub material: M,
}

impl<M: Material> Sphere<M> {
    pub fn new(center: Vec3, radius: f64, material: M) -> Self {
        Self {
            center,
            radius,
            material: material,
        }
    }
}

impl<M: Material> Hittable for Sphere<M> {
    fn hit<'a>(&'a self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord<'a>> {
        let a = ray.direction.dot(ray.direction);
        let b = 2. * ray.direction.dot(ray.origin - self.center);
        let c =
            (ray.origin - self.center).dot(ray.origin - self.center) - self.radius * self.radius;
        let delta = b * b - 4. * a * c;

        let t = (-b - delta.sqrt()) / (2. * a);
        let point = ray.at(t);
        let normal = (point - self.center).unit_vector();
        let normal = if ray.direction.dot(normal) > 0. {
            -normal
        } else {
            normal
        };
        if delta >= 0. && t >= t_min && t <= t_max {
            Some(HitRecord {
                point: point,
                normal: normal,
                t: t,
                material: &self.material,
            })
        } else {
            None
        }
    }
}
