use crate::{matrix4::*, tuples::*};

pub fn translation(x: f64, y: f64, z: f64) -> Matrix4 {
    let m_vec = [
        [1.0, 0.0, 0.0, x],
        [0.0, 1.0, 0.0, y],
        [0.0, 0.0, 1.0, z],
        [0.0, 0.0, 0.0, 1.0]];

    Matrix4::convert(m_vec)
}

pub fn scaling(x: f64, y: f64, z: f64) -> Matrix4 {
    let m_vec = [
        [x, 0.0, 0.0, 0.0],
        [0.0, y, 0.0, 0.0],
        [0.0, 0.0, z, 0.0],
        [0.0, 0.0, 0.0, 1.0]];

    Matrix4::convert(m_vec)
}

pub fn rotation_x(rad: f64) -> Matrix4 {
    let m_vec = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, f64::cos(rad), -1.0 * f64::sin(rad), 0.0],
        [0.0, f64::sin(rad), f64::cos(rad), 0.0],
        [0.0, 0.0, 0.0, 1.0]];

    Matrix4::convert(m_vec)
}

pub fn rotation_y(rad: f64) -> Matrix4 {
    let m_vec = [
        [f64::cos(rad), 0.0, f64::sin(rad), 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [-1.0 * f64::sin(rad), 0.0, f64::cos(rad), 0.0],
        [0.0, 0.0, 0.0, 1.0]];

    Matrix4::convert(m_vec)
}

pub fn rotation_z(rad: f64) -> Matrix4 {
    let m_vec = [
        [f64::cos(rad), -1.0 * f64::sin(rad), 0.0, 0.0],
        [f64::sin(rad), f64::cos(rad), 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0]];

    Matrix4::convert(m_vec)
}

pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix4 {
    let m_vec = [
        [1.0, xy, xz, 0.0],
        [yx, 1.0, yz, 0.0],
        [zx, zy, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0]];

    Matrix4::convert(m_vec)
}

pub fn view_transform(from: Tuple, to: Tuple, up: Tuple) -> Matrix4 {
    let forward = (to - from).normalize();
    let left = forward.xprod(up.normalize());
    let true_up = left.xprod(forward);

    let matrix = Matrix4::convert([
        [left.x, left.y, left.z, 0.0],
        [true_up.x, true_up.y, true_up.z, 0.0],
        [-forward.x, -forward.y, -forward.z, 0.0],
        [0.0, 0.0, 0.0, 1.0]]);

    let translation = translation(-from.x, -from.y, -from.z);

    matrix * translation
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;
    use crate::tuples::*;
    use super::*;

    #[test]
    fn test_translation() {
        let transform = translation(5.0, -3.0, 2.0);
        let p = point(-3.0, 4.0, 5.0);
        assert_eq!(transform * p, point(2.0, 1.0, 7.0));

        let inverse_transform = transform.invert();
        assert_eq!(inverse_transform * p, point(-8.0, 7.0, 3.0));

        let v = vector(-3.0, 4.0, 5.0);
        assert_eq!(transform * v, v);
    }

    #[test]
    fn test_scaling() {
        let transform = scaling(2.0, 3.0, 4.0);
        let p = point(-4.0, 6.0, 8.0);
        assert_eq!(transform * p, point(-8.0, 18.0, 32.0));

        let v = vector(-4.0, 6.0, 8.0);
        assert_eq!(transform * v, vector(-8.0, 18.0, 32.0));

        let inv = transform.invert();
        assert_eq!(inv * v, vector(-2.0, 2.0, 2.0));
    }

    #[test]
    fn test_rotate_x() {
        let p = point(0.0, 1.0, 0.0);
        let half_quarter = rotation_x(PI * 0.25);
        let full_quarter = rotation_x(PI * 0.5);
        assert_eq!(half_quarter * p,
                   point(0.0, 0.5 * f64::sqrt(2.0), 0.5 * f64::sqrt(2.0)));
        assert_eq!(full_quarter * p, point(0.0, 0.0, 1.0));

        let inverse = half_quarter.invert();
        assert_eq!(inverse * p,
                   point(0.0, 0.5 * f64::sqrt(2.0), -0.5 * f64::sqrt(2.0)));
    }

    #[test]
    fn test_rotate_y() {
        let p = point(0.0, 0.0, 1.0);
        let half_quarter = rotation_y(PI * 0.25);
        let full_quarter = rotation_y(PI * 0.5);
        assert_eq!(half_quarter * p,
                   point(f64::sqrt(2.0) * 0.5, 0.0, f64::sqrt(2.0) * 0.5));
        assert_eq!(full_quarter * p, point(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_rotate_z() {
        let p = point(0.0, 1.0, 0.0);
        let half_quarter = rotation_z(PI * 0.25);
        let full_quarter = rotation_z(PI * 0.5);
        assert_eq!(half_quarter * p,
                   point(f64::sqrt(2.0) * -0.5, f64::sqrt(2.0) * 0.5, 0.0));
        assert_eq!(full_quarter * p, point(-1.0, 0.0, 0.0));
    }

    #[test]
    fn test_shearing() {
        let p = point(2.0, 3.0, 4.0);
        let shear1 = shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let shear2 = shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let shear3 = shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let shear4 = shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let shear5 = shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let shear6 = shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);

        assert_eq!(shear1 * p, point(5.0, 3.0, 4.0));
        assert_eq!(shear2 * p, point(6.0, 3.0, 4.0));
        assert_eq!(shear3 * p, point(2.0, 5.0, 4.0));
        assert_eq!(shear4 * p, point(2.0, 7.0, 4.0));
        assert_eq!(shear5 * p, point(2.0, 3.0, 6.0));
        assert_eq!(shear6 * p, point(2.0, 3.0, 7.0));
    }

    #[test]
    fn test_default_view_is_identity_matrix() {
        let from = point(0.0, 0.0, 0.0);
        let to = point(0.0, 0.0, -1.0);
        let up = vector(0.0, 1.0, 0.0);

        let transform = view_transform(from, to, up);
        assert_eq!(transform, Matrix4::identity());
    }

    #[test]
    fn test_view_transformation_turning_around() {
        let from = point(0.0, 0.0, 0.0);
        let to = point(0.0, 0.0, 1.0);
        let up = vector(0.0, 1.0, 0.0);

        let transform = view_transform(from, to, up);
        assert_eq!(transform, scaling(-1.0, 1.0, -1.0));
    }

    #[test]
    fn test_view_transformation_transforms_world() {
        let from = point(0.0, 0.0, 8.0);
        let to = point(0.0, 0.0, 0.0);
        let up = vector(0.0, 1.0, 0.0);

        let transform = view_transform(from, to, up);
        assert_eq!(transform, translation(0.0, 0.0, -8.0));
    }

    #[test]
    fn test_view_transformation() {
        let from = point(1.0, 3.0, 2.0);
        let to = point(4.0, -2.0, 8.0);
        let up = vector(1.0, 1.0, 0.0);

        let transform = view_transform(from, to, up);
        let matrix = [
            [-0.50709, 0.50709, 0.67612, -2.36643],
            [0.76772, 0.60609, 0.12122, -2.82843],
            [-0.35857, 0.59761, -0.71714, 0.00000],
            [0.00000, 0.00000, 0.00000, 1.00000]];

        assert_eq!(transform, Matrix4::convert(matrix));
    }
}