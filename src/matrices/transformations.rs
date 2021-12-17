use crate::matrix4::Matrix4;

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

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;
    use crate::tuples::*;
    use super::*;

    #[test]
    fn test_translation() {
        let transform = translation(5.0, -3.0, 2.0);
        let p = Tuple::point(-3.0, 4.0, 5.0);
        assert_eq!(transform * p, Tuple::point(2.0, 1.0, 7.0));

        let inverse_transform = transform.invert();
        assert_eq!(inverse_transform * p, Tuple::point(-8.0, 7.0, 3.0));

        let v = Tuple::vector(-3.0, 4.0, 5.0);
        assert_eq!(transform * v, v);
    }

    #[test]
    fn test_scaling() {
        let transform = scaling(2.0, 3.0, 4.0);
        let p = Tuple::point(-4.0, 6.0, 8.0);
        assert_eq!(transform * p, Tuple::point(-8.0, 18.0, 32.0));

        let v = Tuple::vector(-4.0, 6.0, 8.0);
        assert_eq!(transform * v, Tuple::vector(-8.0, 18.0, 32.0));

        let inv = transform.invert();
        assert_eq!(inv * v, Tuple::vector(-2.0, 2.0, 2.0));
    }

    #[test]
    fn test_rotate_x() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = rotation_x(PI * 0.25);
        let full_quarter = rotation_x(PI * 0.5);
        assert_eq!(half_quarter * p,
                   Tuple::point(0.0, 0.5 * f64::sqrt(2.0), 0.5 * f64::sqrt(2.0)));
        assert_eq!(full_quarter * p, Tuple::point(0.0, 0.0, 1.0));

        let inverse = half_quarter.invert();
        assert_eq!(inverse * p,
                   Tuple::point(0.0, 0.5 * f64::sqrt(2.0), -0.5 * f64::sqrt(2.0)));
    }

    #[test]
    fn test_rotate_y() {
        let p = Tuple::point(0.0, 0.0, 1.0);
        let half_quarter = rotation_y(PI * 0.25);
        let full_quarter = rotation_y(PI * 0.5);
        assert_eq!(half_quarter * p,
                   Tuple::point(f64::sqrt(2.0) * 0.5, 0.0, f64::sqrt(2.0) * 0.5));
        assert_eq!(full_quarter * p, Tuple::point(1.0, 0.0, 0.0));
    }

    #[test]
    fn test_rotate_z() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = rotation_z(PI * 0.25);
        let full_quarter = rotation_z(PI * 0.5);
        assert_eq!(half_quarter * p,
                   Tuple::point(f64::sqrt(2.0) * -0.5, f64::sqrt(2.0) * 0.5, 0.0));
        assert_eq!(full_quarter * p, Tuple::point(-1.0, 0.0, 0.0));
    }

    #[test]
    fn test_shearing() {
        let p = Tuple::point(2.0, 3.0, 4.0);
        let shear1 = shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let shear2 = shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let shear3 = shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let shear4 = shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let shear5 = shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let shear6 = shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);

        assert_eq!(shear1 * p, Tuple::point(5.0, 3.0, 4.0));
        assert_eq!(shear2 * p, Tuple::point(6.0, 3.0, 4.0));
        assert_eq!(shear3 * p, Tuple::point(2.0, 5.0, 4.0));
        assert_eq!(shear4 * p, Tuple::point(2.0, 7.0, 4.0));
        assert_eq!(shear5 * p, Tuple::point(2.0, 3.0, 6.0));
        assert_eq!(shear6 * p, Tuple::point(2.0, 3.0, 7.0));
    }
}