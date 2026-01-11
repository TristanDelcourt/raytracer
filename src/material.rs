use rand::{Rng, rngs::SmallRng};

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
    fn scatter(&self, ray: Ray, hit: &HitRecord, rng: &mut SmallRng) -> Option<Scatter>;
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
    fn scatter(&self, ray: Ray, hit: &HitRecord, rng: &mut SmallRng) -> Option<Scatter> {
        Some(Scatter {
            ray: Ray::new(
                hit.point,
                reflect(ray.direction, hit.normal) + self.fuzz * random_unit_vector(rng),
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
    fn scatter(&self, _ray: Ray, hit: &HitRecord, rng: &mut SmallRng) -> Option<Scatter> {
        let bounce_dir = hit.normal + random_unit_vector(rng);
        let bounced_ray = Ray::new(hit.point, bounce_dir);

        Some(Scatter {
            ray: bounced_ray,
            attenuation: self.albedo,
        })
    }
}

pub struct Light {
    albedo: Vec3,
    intensity: f64,
}

impl Light {
    pub fn new(albedo: Vec3, intensity: f64) -> Self {
        Self {
            albedo: albedo,
            intensity: intensity,
        }
    }
}

impl Material for Light {
    fn scatter(&self, _ray: Ray, _hit: &HitRecord, _rng: &mut SmallRng) -> Option<Scatter> {
        None
    }

    fn emmited(&self) -> Vec3 {
        self.albedo * self.intensity
    }
}

pub struct Dielectric {
    pub albedo: Vec3,
    pub index: f64,
}

impl Dielectric {
    pub fn new(albedo: Vec3, index: f64) -> Self {
        Self {
            albedo: albedo,
            index: index,
        }
    }
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = ((ref_idx - 1.) / (1.0 + ref_idx)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

impl Material for Dielectric {
    fn scatter(&self, ray: Ray, hit: &HitRecord, rng: &mut SmallRng) -> Option<Scatter> {
        let cos_theta = -hit.normal.dot(ray.direction.unit_vector());
        let ratio = if cos_theta > 0.0 {
            1.0 / self.index
        } else {
            self.index
        };

        let sin2_theta = 1.0 - cos_theta * cos_theta;

        // Total reflection
        if 1. <= ratio * ratio * sin2_theta {
            return Some(Scatter {
                ray: Ray::new(hit.point, reflect(ray.direction, hit.normal)),
                attenuation: self.albedo,
            });
        }

        let reflect_prob = reflectance(cos_theta, ratio);

        if rng.random_range(0.0..1.0) < reflect_prob {
            Some(Scatter {
                ray: Ray::new(hit.point, reflect(ray.direction, hit.normal)),
                attenuation: self.albedo,
            })
        } else {
            let cos_refracted = (1.0 - (ratio * ratio * sin2_theta).powi(2)).sqrt();

            let refracted = if cos_theta >= 0. {
                ratio * ray.direction + (ratio * cos_theta - cos_refracted) * hit.normal
            } else {
                ratio * ray.direction + (ratio * cos_theta + cos_refracted) * hit.normal
            };

            Some(Scatter {
                ray: Ray::new(hit.point, refracted),
                attenuation: self.albedo,
            })
        }
    }
}
