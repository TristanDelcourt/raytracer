use crate::vec3::Vec3;

pub struct Camera {
    pub origin: Vec3,
    pub top_left: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn new(look_from: Vec3, look_at: Vec3, v_up: Vec3, h_fov: f64, aspect_ratio: f64) -> Self {
        let w = (look_from - look_at).unit_vector();
        let u = v_up.cross(w).unit_vector();
        let v = w.cross(u);

        let fov_radians = h_fov * std::f64::consts::PI / 180.0;
        let viewport_width = f64::tan(fov_radians / 2.) * 2.;
        let viewport_height = viewport_width / aspect_ratio;

        let horizontal = u * viewport_width;
        let vertical = v * viewport_height;

        Self {
            origin: look_from,
            top_left: look_from - w - horizontal / 2. + vertical / 2.,
            horizontal: horizontal,
            vertical: vertical,
        }
    }
}
