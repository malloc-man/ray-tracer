use crate::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub enum ObjectHolder {
    Object(Object),
    Group(Group),
}

impl ObjectHolder {
    pub fn from_object(object: Object) -> Self {
        ObjectHolder::Object(object)
    }

    pub fn from_group(group: Group) -> Self {
        ObjectHolder::Group(group)
    }

    pub fn is_group(&self) -> bool {
        if let ObjectHolder::Group(_) = self {
            true
        } else {
            false
        }
    }

    pub fn set_transform(&mut self, transform: Matrix4) -> &mut Self {
        match self {
            ObjectHolder::Object(ref mut object) => object.set_transform(transform),
            ObjectHolder::Group(ref mut group) => group.set_transform(transform),
        }
        self
    }

    pub fn get_transform(&self) -> Matrix4 {
        match self {
            ObjectHolder::Object(object) => object.get_transform(),
            ObjectHolder::Group(group) => group.get_transform(),
        }
    }

    pub fn get_inverse_transform(&self) -> Matrix4 {
        match self {
            ObjectHolder::Object(object) => object.get_inverse_transform(),
            ObjectHolder::Group(group) => group.get_inverse_transform(),
        }
    }

    pub fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        match self {
            ObjectHolder::Object(object) => object.intersect(ray),
            ObjectHolder::Group(group) => group.intersect(ray),
        }
    }

    pub fn add_object_holder(&mut self, element: ObjectHolder) -> Result<(), &str> {
        match self {
            ObjectHolder::Group(ref mut group) => {
                group.add_element(element);
                Ok(())
            },
            _ => Err("Cannot add element to object"),
        }
    }

    pub fn add_object(&mut self, object: Object) -> Result<(), &str> {
        match self {
            ObjectHolder::Group(ref mut group) => {
                group.add_object(object);
                Ok(())
            }
            _ => Err("Cannot add object to object")
        }
    }

    pub fn add_group(&mut self, grp: Group) -> Result<(), &str> {
        match self {
            ObjectHolder::Group(ref mut group) => {
                group.add_group(grp);
                Ok(())
            }
            _ => Err("Cannot add group to object")
        }
    }

    pub fn get_group_members(&self) -> Result<&Vec<ObjectHolder>, &str> {
        match self {
            ObjectHolder::Group(group) => {
                Ok(&group.group)
            }
            _ => Err("Object has no members")
        }
    }

    pub fn get_object(&self) -> Result<&Object, &str> {
        match self {
            ObjectHolder::Object(object) => Ok(object),
            _ => Err("Called get_object on group"),
        }
    }

    pub fn mut_object(&mut self) -> Result<&mut Object, &str> {
        match self {
            ObjectHolder::Object(object) => Ok(object),
            _ => Err("Called get_object on group"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Group {
    group: Vec<ObjectHolder>,
    transform: Matrix4,
    inverse_transform: Matrix4,
    inverse_transform_transposed: Matrix4,
    transformations_list: [f64; 15],
}

impl Group {
    pub fn new(group: Vec<ObjectHolder>) -> Self {
        Self {
            group,
            transform: Matrix4::identity(),
            inverse_transform: Matrix4::identity(),
            inverse_transform_transposed: Matrix4::identity(),
            transformations_list: [
                0.0, 0.0, 0.0,
                1.0, 1.0, 1.0,
                0.0, 0.0, 0.0,
                0.0, 0.0, 0.0,
                0.0, 0.0, 0.0
            ],
        }
    }

    pub fn new_empty() -> Self {
        Self {
            group: vec![],
            transform: Matrix4::identity(),
            inverse_transform: Matrix4::identity(),
            inverse_transform_transposed: Matrix4::identity(),
            transformations_list: [
                0.0, 0.0, 0.0,
                1.0, 1.0, 1.0,
                0.0, 0.0, 0.0,
                0.0, 0.0, 0.0,
                0.0, 0.0, 0.0
            ],
        }
    }

    pub fn add_element(&mut self, element: ObjectHolder) {
        self.group.push(element);
    }

    pub fn add_object(&mut self, object: Object) {
        self.group.push(ObjectHolder::from_object(object));
    }

    pub fn add_group(&mut self, group: Group) {
        self.group.push(ObjectHolder::from_group(group));
    }

    pub fn elements(&self) -> &Vec<ObjectHolder> {
        &self.group
    }

    /* --------------------------- modify transformations --------------------------- */

    pub fn set_transform(&mut self, transform: Matrix4) {
        self.transform = transform;
        self.inverse_transform = transform.invert();
        self.inverse_transform_transposed = self.inverse_transform.transpose();
    }

    fn set_transformation_list(&mut self, index: usize, x: f64) {
        self.transformations_list[index] = x;
        self.update_transform();
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

    pub fn combine_transforms(&mut self, list: [f64; 15]) {
        for i in 0..15 {
            if i <= 2 || i >= 6 {
                self.transformations_list[i] += list[i]
            } else {
                self.transformations_list[i] *= list[i];
            }
        }
        self.update_transform();
    }

    pub fn translate_x(&mut self, x: f64) {
        self.set_transformation_list(0, x);
    }

    pub fn translate_y(&mut self, y: f64) {
        self.set_transformation_list(1, y);
    }

    pub fn translate_z(&mut self, z: f64) {
        self.set_transformation_list(2, z);
    }

    pub fn scale_x(&mut self, x: f64) {
        if x != 0.0 {
            self.set_transformation_list(3, x);
        } else {
            self.set_transformation_list(3, EPSILON);
        }
    }

    pub fn scale_y(&mut self, y: f64) {
        if y != 0.0 {
            self.set_transformation_list(4,  y);
        } else {
            self.set_transformation_list(4, EPSILON);
        }
    }

    pub fn scale_z(&mut self, z: f64) {
        if z != 0.0 {
            self.set_transformation_list(5, z);
        } else {
            self.set_transformation_list(5, EPSILON);
        }
    }

    pub fn rotate_x(&mut self, x: f64) {
        self.set_transformation_list(6, x);
    }

    pub fn rotate_y(&mut self, y: f64) {
        self.set_transformation_list(7,y);
    }

    pub fn rotate_z(&mut self, z: f64) {
        self.set_transformation_list(8, z);
    }

    pub fn shear_xy(&mut self, xy: f64) {
        self.set_transformation_list(9, xy);
    }

    pub fn shear_xz(&mut self, xz: f64) {
        self.set_transformation_list(10, xz);
    }

    pub fn shear_yx(&mut self, yx: f64) {
        self.set_transformation_list(11, yx);
    }

    pub fn shear_yz(&mut self, yz: f64) {
        self.set_transformation_list(12, yz);
    }

    pub fn shear_zx(&mut self, zx: f64) {
        self.set_transformation_list(13, zx);
    }

    pub fn shear_zy(&mut self, zy: f64) {
        self.set_transformation_list(14, zy);
    }

    /* --------------------------- access transformations --------------------------- */

    pub fn get_transform(&self) -> Matrix4 {
        self.transform
    }

    pub fn get_inverse_transform(&self) -> Matrix4 {
        self.inverse_transform
    }

    pub fn get_inverse_transform_transposed(&self) -> Matrix4 {
        self.inverse_transform_transposed
    }

    pub fn get_transformation_list(&self, index: usize) -> f64 {
        self.transformations_list[index]
    }

    pub fn transformation_list_all(&self) -> [f64; 15] {
        self.transformations_list
    }

    pub fn get_translate_x(&self) -> f64 {
        self.get_transformation_list(0)
    }

    pub fn get_translate_y(&self) -> f64 {
        self.get_transformation_list(1)
    }

    pub fn get_translate_z(&self) -> f64 {
        self.get_transformation_list(2)
    }

    pub fn get_scale_x(&self) -> f64 {
        self.get_transformation_list(3)
    }

    pub fn get_scale_y(&self) -> f64 {
        self.get_transformation_list(4)
    }

    pub fn get_scale_z(&self) -> f64 {
        self.get_transformation_list(5)
    }

    pub fn get_rotate_x(&self) -> f64 {
        self.get_transformation_list(6)
    }

    pub fn get_rotate_y(&self) -> f64 {
        self.get_transformation_list(7)
    }

    pub fn get_rotate_z(&self) -> f64 {
        self.get_transformation_list(8)
    }

    pub fn get_shear_xy(&self) -> f64 {
        self.get_transformation_list(9)
    }

    pub fn get_shear_xz(&self) -> f64 {
        self.get_transformation_list(10)
    }

    pub fn get_shear_yx(&self) -> f64 {
        self.get_transformation_list(11)
    }

    pub fn get_shear_yz(&self) -> f64 {
        self.get_transformation_list(12)
    }

    pub fn get_shear_zx(&self) -> f64 {
        self.get_transformation_list(13)
    }

    pub fn get_shear_zy(&self) -> f64 {
        self.get_transformation_list(14)
    }

    /* --------------------------- ray tracing calculations --------------------------- */

    pub fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        let local_ray = ray.transform(self.get_inverse_transform());
        self.local_intersect(local_ray)
    }

    fn local_intersect(&self, ray: Ray) -> Vec<Intersection> {
        let mut vec = vec![];
        for object in &self.group {
            for i in object.intersect(ray) {
                let mut new_obj = i.get_object();
                new_obj.combine_transforms(self.transformations_list);
                vec.push(Intersection::new(i.get_t(), new_obj));
            }
        }
        vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
        vec
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_adding_child_to_group() {
        let mut g = ObjectHolder::Group(Group::new_empty());
        g.add_object(spheres::new()).unwrap();
        assert_eq!(g.get_group_members().unwrap().len(), 1);
        assert_eq!(g.get_group_members().unwrap()[0].get_object().unwrap(), &spheres::new());
    }

    #[test]
    fn test_intersect_empty_group() {
        let g = ObjectHolder::Group(Group::new_empty());
        let r = Ray::new(origin(), vector(0.0, 0.0, 1.0));
        let xs = g.intersect(r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn test_intersect_nonempty_group() {
        let grp = Group::new_empty();
        let mut g = ObjectHolder::from_group(grp);

        let mut s1 = ObjectHolder::from_object(spheres::new());
        s1.mut_object().unwrap().set_color(color(0.0, 0.0, 1.0));
        let mut s2 = ObjectHolder::from_object(spheres::new());
        s2.mut_object().unwrap().translate_z(-3.0);
        s2.mut_object().unwrap().set_color(color(1.0, 0.0, 0.0));
        let mut s3 = ObjectHolder::from_object(spheres::new());
        s3.mut_object().unwrap().translate_x(5.0);
        s3.mut_object().unwrap().set_color(color(0.0, 1.0, 0.0));

        let s_1 = s1.clone();
        let s_2 = s2.clone();

        g.add_object_holder(s1).unwrap();
        g.add_object_holder(s2).unwrap();
        g.add_object_holder(s3).unwrap();

        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let xs = g.intersect(r);

        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].get_object(), *s_2.get_object().unwrap());
        assert_eq!(xs[1].get_object(), *s_2.get_object().unwrap());
        assert_eq!(xs[2].get_object(), *s_1.get_object().unwrap());
        assert_eq!(xs[3].get_object(), *s_1.get_object().unwrap());
    }

    #[test]
    fn test_group_transformations() {
        let mut g = Group::new_empty();
        g.scale_x(2.0);
        g.scale_y(2.0);
        g.scale_z(2.0);

        let mut g = ObjectHolder::from_group(g);

        let mut s = spheres::new();
        s.translate_x(5.0);

        g.add_object(s);

        let r = Ray::new(point(10.0, 0.0, -10.0), vector(0.0, 0.0, 1.0));
        let xs = g.intersect(r);
        assert_eq!(xs.len(), 2);
    }
}