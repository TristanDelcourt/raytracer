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
    fn scatter(&self, ray: Ray, hit: &HitRecord) -> Option<Scatter>;
    fn emmited(&self) -> Vec3 {
        Vec3::new(0., 0., 0.)
    }
}

pub struct Reflective {
    pub albedo: Vec3,
    pub fuzz: f64,
}

impl Reflective {
    pub fn new(albedo: Vec3, fuzz: f64) -> Self {
        Self {
            albedo: albedo,
            fuzz: fuzz,
        }
    }
}

impl Material for Reflective {
    fn scatter(&self, ray: Ray, hit: &HitRecord) -> Option<Scatter> {
        Some(Scatter {
            ray: Ray::new(
                hit.point,
                reflect(ray.direction, hit.normal) + self.fuzz * random_unit_vector(),
            ),
            attenuation: self.albedo,
        })
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
    fn scatter(&self, _ray: Ray, hit: &HitRecord) -> Option<Scatter> {
        let bounce_dir = hit.normal + random_unit_vector();
        let bounced_ray = Ray::new(hit.point, bounce_dir);

        Some(Scatter {
            ray: bounced_ray,
            attenuation: self.albedo,
        })
    }
}

pub struct Light {
    albedo: Vec3,
}

impl Light {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo: albedo }
    }
}

impl Material for Light {
    fn scatter(&self, _ray: Ray, _hit: &HitRecord) -> Option<Scatter> {
        None
    }

    fn emmited(&self) -> Vec3 {
        self.albedo
    }
}
