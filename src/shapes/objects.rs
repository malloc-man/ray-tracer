use crate::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Object {
    material: Material,
    transform: Matrix4,
    inverse_transform: Matrix4,
    inverse_transform_transposed: Matrix4,
    pub shape: Shape,
    transformations_list: [f64; 15],
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Shape {
    Sphere,
    Plane,
    Cube,
    Cylinder {min: f64, max: f64, closed: bool},
    Cone {min: f64, max: f64, closed: bool},
}

impl std::fmt::Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Shape::Sphere => write!(f, "Sphere"),
            Shape::Plane => write!(f, "Plane"),
            Shape::Cube => write!(f, "Cube"),
            Shape::Cylinder {min: _, max: _, closed: _} => write!(f, "Cylinder"),
            Shape::Cone {min: _, max: _, closed: _} => write!(f, "Cone"),
        }
    }
}

/* ------------------------------------------------------------------------------------ */

impl Object {
    pub fn new(shape: Shape) -> Self {
        Self {
            material: Material::new(),
            transform: Matrix4::identity(),
            inverse_transform: Matrix4::identity(),
            inverse_transform_transposed: Matrix4::identity(),
            shape,
            transformations_list: [
                0.0, 0.0, 0.0,
                1.0, 1.0, 1.0,
                0.0, 0.0, 0.0,
                0.0, 0.0, 0.0,
                0.0, 0.0, 0.0
            ],
        }
    }

    /* --------------------------- general transformations --------------------------- */

    pub fn set_transform(&mut self, transform: Matrix4) -> &Self {
        self.transform = transform;
        self.inverse_transform = transform.invert();
        self.inverse_transform_transposed = self.inverse_transform.transpose();
        self
    }

    pub fn get_transform(&self) -> Matrix4 {
        self.transform
    }

    pub fn update_transform(&mut self) {
        let translation = translation(
            self.get_translate_x(),
            self.get_translate_y(),
            self.get_translate_z(),
        );
        let scaling = scaling(
            self.get_scale_x(),
            self.get_scale_y(),
            self.get_scale_z(),
        );
        let rotation_x = rotation_x(self.get_rotate_x());
        let rotation_y = rotation_y(self.get_rotate_y());
        let rotation_z = rotation_z(self.get_rotate_z());

        let shear = shearing(
            self.get_shear_xy(),
            self.get_shear_xz(),
            self.get_shear_yx(),
            self.get_shear_yz(),
            self.get_shear_zx(),
            self.get_shear_zy(),
        );

        self.set_transform(translation * scaling * rotation_x * rotation_y * rotation_z * shear);
    }

    pub fn get_inverse_transform(&self) -> Matrix4 {
        self.inverse_transform
    }

    pub fn get_inverse_transform_transposed(&self) -> Matrix4 {
        self.inverse_transform_transposed
    }

    /* --------------------------- modify transformations --------------------------- */

    pub fn translate_x(&mut self, x: f64) {
        self.transformations_list[0] = x;
        self.update_transform();
    }

    pub fn translate_y(&mut self, y: f64) {
        self.transformations_list[1] = y;
        self.update_transform();
    }

    pub fn translate_z(&mut self, z: f64) {
        self.transformations_list[2] = z;
        self.update_transform();
    }

    pub fn scale_x(&mut self, x: f64) {
        if x != 0.0 {
            self.transformations_list[3] = x;
        } else {
            self.transformations_list[3] = EPSILON;
        }
        self.update_transform();
    }

    pub fn scale_y(&mut self, y: f64) {
        if y != 0.0 {
            self.transformations_list[4] = y;
        } else {
            self.transformations_list[4] = EPSILON;
        }
        self.update_transform();
    }

    pub fn scale_z(&mut self, z: f64) {
        if z != 0.0 {
            self.transformations_list[5] = z
        } else {
            self.transformations_list[5] = EPSILON;
        }
        self.update_transform();
    }

    pub fn rotate_x(&mut self, x: f64) {
        self.transformations_list[6] = x;
        self.update_transform();
    }

    pub fn rotate_y(&mut self, y: f64) {
        self.transformations_list[7] = y;
        self.update_transform();
    }

    pub fn rotate_z(&mut self, z: f64) {
        self.transformations_list[8] = z;
        self.update_transform();
    }

    pub fn shear_xy(&mut self, xy: f64) {
        self.transformations_list[9] = xy;
        self.update_transform();
    }

    pub fn shear_xz(&mut self, xz: f64) {
        self.transformations_list[10] = xz;
        self.update_transform();
    }

    pub fn shear_yx(&mut self, yx: f64) {
        self.transformations_list[11] = yx;
        self.update_transform();
    }

    pub fn shear_yz(&mut self, yz: f64) {
        self.transformations_list[12] = yz;
        self.update_transform();
    }

    pub fn shear_zx(&mut self, zx: f64) {
        self.transformations_list[13] = zx;
        self.update_transform();
    }

    pub fn shear_zy(&mut self, zy: f64) {
        self.transformations_list[14] = zy;
        self.update_transform();
    }

    /* --------------------------- access transformations --------------------------- */

    pub fn get_translate_x(&self) -> f64 {
        self.transformations_list[0]
    }

    pub fn get_translate_y(&self) -> f64 {
        self.transformations_list[1]
    }

    pub fn get_translate_z(&self) -> f64 {
        self.transformations_list[2]
    }

    pub fn get_scale_x(&self) -> f64 {
        self.transformations_list[3]
    }

    pub fn get_scale_y(&self) -> f64 {
        self.transformations_list[4]
    }

    pub fn get_scale_z(&self) -> f64 {
        self.transformations_list[5]
    }

    pub fn get_rotate_x(&self) -> f64 {
        self.transformations_list[6]
    }

    pub fn get_rotate_y(&self) -> f64 {
        self.transformations_list[7]
    }

    pub fn get_rotate_z(&self) -> f64 {
        self.transformations_list[8]
    }

    pub fn get_shear_xy(&self) -> f64 {
        self.transformations_list[9]
    }

    pub fn get_shear_xz(&self) -> f64 {
        self.transformations_list[10]
    }

    pub fn get_shear_yx(&self) -> f64 {
        self.transformations_list[11]
    }

    pub fn get_shear_yz(&self) -> f64 {
        self.transformations_list[12]
    }

    pub fn get_shear_zx(&self) -> f64 {
        self.transformations_list[13]
    }

    pub fn get_shear_zy(&self) -> f64 {
        self.transformations_list[14]
    }

    /* --------------------------- get material attributes --------------------------- */

    pub fn get_material(&self) -> Material {
        self.material
    }

    pub fn get_color(&self) -> Color {
        self.material.get_color()
    }

    pub fn get_ambient(&self) -> f64 {
        self.material.get_ambient()
    }

    pub fn get_diffuse(&self) -> f64 {
        self.material.get_diffuse()
    }

    pub fn get_specular(&self) -> f64 {
        self.material.get_specular()
    }

    pub fn get_shininess(&self) -> f64 {
        self.material.get_shininess()
    }

    pub fn get_pattern(&self) -> Pattern {
        self.material.get_pattern()
    }

    pub fn get_pattern_transform(&self) -> Matrix4 {
        self.material.get_pattern_transform()
    }

    pub fn get_pattern_inverse_transform(&self) -> Matrix4 {
        self.material.get_pattern_inverse_transform()
    }

    pub fn get_reflective(&self) -> f64 {
        self.material.get_reflective()
    }

    pub fn get_transparency(&self) -> f64 {
        self.material.get_transparency()
    }

    pub fn get_refractive_index(&self) -> f64 {
        self.material.get_refractive_index()
    }

    pub fn casts_shadow(&self) -> bool {
        self.material.casts_shadow()
    }

    /* --------------------------- set material attributes --------------------------- */

    pub fn set_material(&mut self, material: Material) -> &mut Self {
        self.material = material;
        self
    }

    pub fn set_color(&mut self, color: Color) -> &mut Self {
        self.material.set_color(color);
        self
    }

    pub fn set_ambient(&mut self, ambient: f64) -> &mut Self {
        self.material.set_ambient(ambient);
        self
    }

    pub fn set_diffuse(&mut self, diffuse: f64) -> &mut Self {
        self.material.set_diffuse(diffuse);
        self
    }

    pub fn set_specular(&mut self, specular: f64) -> &mut Self {
        self.material.set_specular(specular);
        self
    }

    pub fn set_shininess(&mut self, shininess: f64) -> &mut Self {
        self.material.set_shininess(shininess);
        self
    }

    pub fn set_pattern(&mut self, pattern: Pattern) -> &mut Self {
        self.material.set_pattern(pattern);
        self
    }

    pub fn set_pattern_transform(&mut self, transform: Matrix4) -> &mut Self {
        self.material.set_pattern_transform(transform);
        self
    }

    pub fn set_reflective(&mut self, reflective: f64) -> &mut Self {
        self.material.set_reflective(reflective);
        self
    }

    pub fn set_transparency(&mut self, transparency: f64) -> &mut Self {
        self.material.set_transparency(transparency);
        self
    }

    pub fn set_refractive_index(&mut self, index: f64) -> &mut Self {
        self.material.set_refractive_index(index);
        self
    }

    pub fn set_casts_shadow(&mut self, cs: bool) -> &mut Self {
        self.material.set_casts_shadow(cs);
        self
    }

    /* --------------------------- ray tracing calculations --------------------------- */

    pub fn normal_at(&self, pt: Tuple) -> Tuple {
        let local_point = self.inverse_transform * pt;
        let local_normal = match self.shape {
            Shape::Sphere => spheres::normal_at(local_point),
            Shape::Plane => planes::normal_at(),
            Shape::Cube => cubes::normal_at(local_point),
            Shape::Cylinder {min: _, max: _, closed: _} => cylinders::normal_at(*self, local_point),
            Shape::Cone {min: _, max: _, closed: _} => cones::normal_at(*self, local_point),
        };
        let world_normal = self.inverse_transform_transposed * local_normal;
        world_normal.vectorize().normalize()
    }

    pub fn intersect(self, ray: Ray) -> Vec<Intersection> {
        let local_ray = ray.transform(self.get_inverse_transform());
        match self.shape {
            Shape::Sphere => spheres::intersect(self, local_ray),
            Shape::Plane => planes::intersect(self, local_ray),
            Shape::Cube => cubes::intersect(self, local_ray),
            Shape::Cylinder {min: _, max: _, closed: _} => cylinders::intersect(self, local_ray),
            Shape::Cone {min: _, max: _, closed: _} => cones::intersect(self, local_ray),
        }
    }

    pub fn pattern_at_object(&self, point: Tuple) -> Color {
        if self.get_pattern().get_pattern_type() == PatternType::Solid {
            return self.get_color();
        }
        let local_point = self.inverse_transform * point;
        let pattern_space_point = self.get_pattern_inverse_transform() * local_point;
        self.get_pattern().pattern_at(pattern_space_point)
    }
}
