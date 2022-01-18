use crate::prelude::*;

pub trait Transformable {
    fn set_transform(&mut self, transform: Matrix4);

    fn set_transformation_list(&mut self, index: usize, x: f64);

    fn update_transform(&mut self) {
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

    fn combine_transforms(&mut self, list: [f64; 15]) {
        for i in 0..15 {
            if i <= 2 || i >= 6 {
                self.transformation_list_ref()[i] += list[i]
            } else {
                self.transformation_list_ref()[i] *= list[i];
            }
        }
        self.update_transform();
    }

    fn translate_x(&mut self, x: f64) {
        self.set_transformation_list(0, x);
    }

    fn translate_y(&mut self, y: f64) {
        self.set_transformation_list(1, y);
    }

    fn translate_z(&mut self, z: f64) {
        self.set_transformation_list(2, z);
    }

    fn scale_x(&mut self, x: f64) {
        if x != 0.0 {
            self.set_transformation_list(3, x);
        } else {
            self.set_transformation_list(3, EPSILON);
        }
    }

    fn scale_y(&mut self, y: f64) {
        if y != 0.0 {
            self.set_transformation_list(4,  y);
        } else {
            self.set_transformation_list(4, EPSILON);
        }
    }

    fn scale_z(&mut self, z: f64) {
        if z != 0.0 {
            self.set_transformation_list(5, z);
        } else {
            self.set_transformation_list(5, EPSILON);
        }
    }

    fn rotate_x(&mut self, x: f64) { self.set_transformation_list(6, x); }

    fn rotate_y(&mut self, y: f64) { self.set_transformation_list(7,y); }

    fn rotate_z(&mut self, z: f64) { self.set_transformation_list(8, z); }

    fn shear_xy(&mut self, xy: f64) { self.set_transformation_list(9, xy); }

    fn shear_xz(&mut self, xz: f64) { self.set_transformation_list(10, xz); }

    fn shear_yx(&mut self, yx: f64) { self.set_transformation_list(11, yx); }

    fn shear_yz(&mut self, yz: f64) { self.set_transformation_list(12, yz); }

    fn shear_zx(&mut self, zx: f64) { self.set_transformation_list(13, zx); }

    fn shear_zy(&mut self, zy: f64) { self.set_transformation_list(14, zy); }

    /* --------------------------- access transformations --------------------------- */

    fn get_transform(&self) -> Matrix4;

    fn get_inverse_transform(&self) -> Matrix4;

    fn get_inverse_transform_transposed(&self) -> Matrix4;

    fn get_transformation_list(&self, index: usize) -> f64;

    fn transformation_list_all(&self) -> [f64; 15];

    fn transformation_list_ref(&mut self) -> &mut [f64; 15];

    fn get_translate_x(&self) -> f64 {
        self.get_transformation_list(0)
    }

    fn get_translate_y(&self) -> f64 {
        self.get_transformation_list(1)
    }

    fn get_translate_z(&self) -> f64 {
        self.get_transformation_list(2)
    }

    fn get_scale_x(&self) -> f64 {
        self.get_transformation_list(3)
    }

    fn get_scale_y(&self) -> f64 {
        self.get_transformation_list(4)
    }

    fn get_scale_z(&self) -> f64 {
        self.get_transformation_list(5)
    }

    fn get_rotate_x(&self) -> f64 {
        self.get_transformation_list(6)
    }

    fn get_rotate_y(&self) -> f64 {
        self.get_transformation_list(7)
    }

    fn get_rotate_z(&self) -> f64 {
        self.get_transformation_list(8)
    }

    fn get_shear_xy(&self) -> f64 {
        self.get_transformation_list(9)
    }

    fn get_shear_xz(&self) -> f64 {
        self.get_transformation_list(10)
    }

    fn get_shear_yx(&self) -> f64 {
        self.get_transformation_list(11)
    }

    fn get_shear_yz(&self) -> f64 {
        self.get_transformation_list(12)
    }

    fn get_shear_zx(&self) -> f64 {
        self.get_transformation_list(13)
    }

    fn get_shear_zy(&self) -> f64 {
        self.get_transformation_list(14)
    }
}