use crate::prelude::*;
use crate::shapes::traits::Transformable;

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

impl Transformable for Group {
    fn set_transform(&mut self, transform: Matrix4) {
        self.transform = transform;
        self.inverse_transform = transform.invert();
        self.inverse_transform_transposed = self.inverse_transform.transpose();
    }

    fn set_transformation_list(&mut self, index: usize, x: f64) {
        self.transformations_list[index] = x;
        self.update_transform();
    }

    fn get_transform(&self) -> Matrix4 {
        self.transform
    }

    fn get_inverse_transform(&self) -> Matrix4 {
        self.inverse_transform
    }

    fn get_inverse_transform_transposed(&self) -> Matrix4 {
        self.inverse_transform_transposed
    }

    fn get_transformation_list(&self, index: usize) -> f64 {
        self.transformations_list[index]
    }

    fn transformation_list_all(&self) -> [f64; 15] {
        self.transformations_list
    }

    fn transformation_list_ref(&mut self) -> &mut [f64; 15] {
        &mut self.transformations_list
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