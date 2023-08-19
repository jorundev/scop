use super::{
    matrix::Mat4,
    vec::{Vec3, Vec4},
};
use std::ops::*;

#[derive(Debug, Clone, Copy)]
pub struct Quaternion {
    inner: Vec4,
}

impl Quaternion {
    pub fn new() -> Self {
        Self {
            inner: Vec4(0.0, 0.0, 0.0, 1.0),
        }
    }

    pub fn new_with_vector(vec: Vec3) -> Self {
        Self {
            inner: Vec4(vec.0, vec.1, vec.2, 0.0),
        }
    }

    pub fn from_rotation_y(angle: f32) -> Self {
        let angle = angle.to_radians();
        let half_angle = angle * 0.5;

        let imaginary = Vec3(0.0, half_angle.sin(), 0.0);
        let scalar = half_angle.cos();

        Self {
            inner: Vec4(imaginary.0, imaginary.1, imaginary.2, scalar),
        }
    }

    pub fn from_rotation_x(angle: f32) -> Self {
        let angle = angle.to_radians();
        let half_angle = angle * 0.5;

        let imaginary = Vec3(half_angle.sin(), 0.0, 0.0);
        let scalar = half_angle.cos();

        Self {
            inner: Vec4(imaginary.0, imaginary.1, imaginary.2, scalar),
        }
    }

    pub fn from_rotation_z(angle: f32) -> Self {
        let angle = angle.to_radians();
        let half_angle = angle * 0.5;

        let imaginary = Vec3(0.0, 0.0, half_angle.sin());
        let scalar = half_angle.cos();

        Self {
            inner: Vec4(imaginary.0, imaginary.1, imaginary.2, scalar),
        }
    }

    pub fn as_vec4(&self) -> &Vec4 {
        &self.inner
    }

    pub fn as_vec4_mut(&mut self) -> &mut Vec4 {
        &mut self.inner
    }

    pub fn mul_with_vec3(&self, vector: &Vec3) -> Vec3 {
        let qv = self.imaginary_vector();
        let w = self.scalar();

        let term1 = qv * vector;
        let term2 = Vec3(-w * vector.0, -w * vector.1, -w * vector.2);
        let term3 = qv.cross(*vector) * 2.0;

        term1 + term2 + term3
    }

    // Real part
    pub fn scalar(&self) -> f32 {
        self.inner.3
    }

    // Imaginary part
    pub fn imaginary_vector(&self) -> Vec3 {
        Vec3(self.inner.0, self.inner.1, self.inner.2)
    }

    pub fn conjugate(&self) -> Self {
        Self {
            inner: Vec4(-self.inner.0, -self.inner.1, -self.inner.2, self.inner.3),
        }
    }

    // Rotate using quaternion
    // 'vec' is a normalized direction vector starting from the rotation center
    // Imagine 'vec' as a point that rotates around (0, 0, 0) using the quaternion's axis and angle
    // The resulting vector points from (0, 0, 0) to the point's new position in space after rotation
    pub fn rotate(&self, vec: Vec3) -> Vec3 {
        let vector_quaternion = Self::new_with_vector(vec);
        let result_quat = self * vector_quaternion * self.conjugate();
        result_quat.imaginary_vector()
    }

    pub fn rotation_matrix(&self) -> Mat4 {
        let Vec3(x, y, z) = self.imaginary_vector();
        let w = self.scalar();

        let xx = x * x;
        let xy = x * y;
        let xz = x * z;
        let xw = x * w;

        let yy = y * y;
        let yz = y * z;
        let yw = y * w;

        let zz = z * z;
        let zw = z * w;

        Mat4::from_cols(
            Vec4(1.0 - 2.0 * (yy + zz), 2.0 * (xy - zw), 2.0 * (xz + yw), 0.0),
            Vec4(2.0 * (xy + zw), 1.0 - 2.0 * (xx + zz), 2.0 * (yz - xw), 0.0),
            Vec4(2.0 * (xz - yw), 2.0 * (yz + xw), 1.0 - 2.0 * (xx + yy), 0.0),
            Vec4(0.0, 0.0, 0.0, 1.0),
        )
    }

    pub fn from_rotation_matrix(matrix: Mat4) -> Quaternion {
        let trace = matrix.x_axis.0 + matrix.y_axis.1 + matrix.z_axis.2;

        if trace > 0.0 {
            let s = 0.5 / (trace + 1.0).sqrt();
            Quaternion {
                inner: Vec4(
                    (matrix.z_axis.1 - matrix.y_axis.2) * s,
                    (matrix.x_axis.2 - matrix.z_axis.0) * s,
                    (matrix.y_axis.0 - matrix.x_axis.1) * s,
                    0.25 / s,
                ),
            }
        } else if matrix.x_axis.0 > matrix.y_axis.1 && matrix.x_axis.0 > matrix.z_axis.2 {
            let s = 2.0 * (1.0 + matrix.x_axis.0 - matrix.y_axis.1 - matrix.z_axis.2).sqrt();
            Quaternion {
                inner: Vec4(
                    0.25 * s,
                    (matrix.y_axis.0 + matrix.x_axis.1) / s,
                    (matrix.z_axis.0 + matrix.x_axis.2) / s,
                    (matrix.z_axis.1 - matrix.y_axis.2) / s,
                ),
            }
        } else if matrix.y_axis.1 > matrix.z_axis.2 {
            let s = 2.0 * (1.0 + matrix.y_axis.1 - matrix.x_axis.0 - matrix.z_axis.2).sqrt();
            Quaternion {
                inner: Vec4(
                    (matrix.y_axis.0 + matrix.x_axis.1) / s,
                    0.25 * s,
                    (matrix.z_axis.1 + matrix.y_axis.2) / s,
                    (matrix.x_axis.2 - matrix.z_axis.0) / s,
                ),
            }
        } else {
            let s = 2.0 * (1.0 + matrix.z_axis.2 - matrix.x_axis.0 - matrix.y_axis.1).sqrt();
            Quaternion {
                inner: Vec4(
                    (matrix.z_axis.0 + matrix.x_axis.2) / s,
                    (matrix.z_axis.1 + matrix.y_axis.2) / s,
                    0.25 * s,
                    (matrix.y_axis.0 - matrix.x_axis.1) / s,
                ),
            }
        }
    }

    // TODO: broken (from_rotation_matrix too probably)
    // Create a rotation quaternion that aligns the forward vector with the given direction and up vector
    pub fn look_rotation(forward: Vec3, up: Vec3) -> Quaternion {
        let right = forward.cross(up).normalize();
        let new_up = right.cross(forward);

        let rotation_matrix = Mat4 {
            x_axis: Vec4(right.0, right.1, right.2, 0.0),
            y_axis: Vec4(new_up.0, new_up.1, new_up.2, 0.0),
            z_axis: Vec4(-forward.0, -forward.1, -forward.2, 0.0),
            w_axis: Vec4(0.0, 0.0, 0.0, 1.0),
        };

        Quaternion::from_rotation_matrix(rotation_matrix)
    }

    pub fn from_directions(from: Vec3, to: Vec3) -> Quaternion {
        let imaginary = from.cross(to);
        let dot = from.dot(to);
        let scalar = (from.length_squared() * to.length_squared()).sqrt() + dot;

        Quaternion {
            inner: Vec4(imaginary.0, imaginary.1, imaginary.2, scalar).normalize(),
        }
    }

    pub fn inverse(&self) -> Quaternion {
        let norm_squared = self.inner.length_squared();

        if norm_squared == 0.0 {
            todo!("Cannot inverse zero-lenght quaternion");
        }

        let factor = 1.0 / norm_squared;

        Quaternion {
            inner: Vec4(
                -self.inner.0 * factor,
                -self.inner.1 * factor,
                -self.inner.2 * factor,
                self.inner.3 * factor,
            ),
        }
    }
}

impl Mul<Quaternion> for Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: Quaternion) -> Quaternion {
        let Vec4(x1, y1, z1, w1) = self.inner;
        let Vec4(x2, y2, z2, w2) = rhs.inner;

        let w = w1 * w2 - x1 * x2 - y1 * y2 - z1 * z2;
        let x = w1 * x2 + x1 * w2 + y1 * z2 - z1 * y2;
        let y = w1 * y2 - x1 * z2 + y1 * w2 + z1 * x2;
        let z = w1 * z2 + x1 * y2 - y1 * x2 + z1 * w2;

        Quaternion {
            inner: Vec4(x, y, z, w),
        }
    }
}

impl<'a> Mul<Quaternion> for &'a Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: Quaternion) -> Quaternion {
        (*self).clone() * rhs
    }
}

impl Mul<f32> for Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: f32) -> Self::Output {
        Quaternion {
            inner: Vec4(
                self.inner.0 * rhs,
                self.inner.1 * rhs,
                self.inner.2 * rhs,
                self.inner.3 * rhs,
            ),
        }
    }
}

impl Div<f32> for Quaternion {
    type Output = Quaternion;

    fn div(self, rhs: f32) -> Self::Output {
        Quaternion {
            inner: Vec4(
                self.inner.0 / rhs,
                self.inner.1 / rhs,
                self.inner.2 / rhs,
                self.inner.3 / rhs,
            ),
        }
    }
}

impl std::fmt::Display for Quaternion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Vec3(x, y, z) = self.imaginary_vector();
        let scalar = self.scalar();
        write!(f, "(({x}, {y}, {z}), {scalar})")
    }
}

impl Neg for Quaternion {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        self.inner.3 = -self.inner.3;
        self
    }
}
