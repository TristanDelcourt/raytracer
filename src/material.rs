use crate::{
    hittable::HitRecord,
    ray::Ray,
    vec3::{Vec3, random_unit_vector, reflect},
};

pub struct Scatter {
    pub ray: Ray,
    pub attenuation: Vec3,
}

pub trait Material {
    fn scatter(&self, ray: Ray, hit: &HitRecord) -> Scatter;
}

pub struct Metal {
    pub albedo: Vec3,
}

impl Metal {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: Ray, hit: &HitRecord) -> Scatter {
        Scatter {
            ray: Ray::new(hit.point, reflect(ray.direction, hit.normal)),
            attenuation: self.albedo,
        }
    }
}

pub struct Diffuse {
    pub albedo: Vec3,
}

impl Diffuse {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Material for Diffuse {
    fn scatter(&self, _ray: Ray, hit: &HitRecord) -> Scatter {
        let bounce_dir = hit.normal + random_unit_vector();
        let bounced_ray = Ray::new(hit.point, bounce_dir);

        Scatter {
            ray: bounced_ray,
            attenuation: self.albedo,
        }
    }
}
