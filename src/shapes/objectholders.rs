use crate::prelude::*;
use crate::shapes::traits::Transformable;

#[derive(Clone, Debug, PartialEq)]
pub enum ObjectHolder {
    Object(Object),
    Group(Group),
}

impl std::fmt::Display for ObjectHolder {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ObjectHolder::Object(object) => object.shape.fmt(f),
            ObjectHolder::Group(_) => write!(f, "Group"),
        }
    }
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
                Ok(&group.elements())
            }
            _ => Err("Object has no members")
        }
    }

    pub fn remove_from_group(&mut self, index: usize) -> Result<(), &str> {
        match self {
            ObjectHolder::Group(group) => {
                group.mut_elements().remove(index);
                Ok(())
            }
            _ => Err("Attempted to remove object from an object. Can only remove from group")
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

impl Transformable for ObjectHolder {
    fn set_transform(&mut self, transform: Matrix4) {
        match self {
            ObjectHolder::Object(ref mut object) => object.set_transform(transform),
            ObjectHolder::Group(ref mut group) => group.set_transform(transform),
        }
    }

    fn set_transformation_list(&mut self, index: usize, x: f64) {
        match self {
            ObjectHolder::Object(ref mut object) => object.set_transformation_list(index, x),
            ObjectHolder::Group(ref mut group) => group.set_transformation_list(index, x),
        }
    }

    fn get_transform(&self) -> Matrix4 {
        match self {
            ObjectHolder::Object(object) => object.get_transform(),
            ObjectHolder::Group(group) => group.get_transform(),
        }
    }

    fn get_inverse_transform(&self) -> Matrix4 {
        match self {
            ObjectHolder::Object(object) => object.get_inverse_transform(),
            ObjectHolder::Group(group) => group.get_inverse_transform(),
        }
    }

    fn get_inverse_transform_transposed(&self) -> Matrix4 {
        match self {
            ObjectHolder::Object(object) => object.get_inverse_transform_transposed(),
            ObjectHolder::Group(group) => group.get_inverse_transform_transposed(),
        }
    }

    fn get_transformation_list(&self, index: usize) -> f64 {
        match self {
            ObjectHolder::Object(object) => object.get_transformation_list(index),
            ObjectHolder::Group(group) => group.get_transformation_list(index),
        }
    }

    fn transformation_list_all(&self) -> [f64; 15] {
        match self {
            ObjectHolder::Object(object) => object.transformation_list_all(),
            ObjectHolder::Group(group) => group.transformation_list_all(),
        }
    }

    fn transformation_list_ref(&mut self) -> &mut [f64; 15] {
        match self {
            ObjectHolder::Object(ref mut object) => object.transformation_list_ref(),
            ObjectHolder::Group(ref mut group) => group.transformation_list_ref(),
        }
    }
}