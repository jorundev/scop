use super::{
    quaternion::Quaternion,
    vec::{Vec3, Vec4},
};

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Mat4 {
    pub x_axis: Vec4,
    pub y_axis: Vec4,
    pub z_axis: Vec4,
    pub w_axis: Vec4,
}

impl Mat4 {
    pub const ZERO: Self = Self::from_cols(Vec4::ZERO, Vec4::ZERO, Vec4::ZERO, Vec4::ZERO);
    pub const IDENTITY: Self = Self::from_cols(Vec4::X, Vec4::Y, Vec4::Z, Vec4::W);

    pub const fn from_cols(x_axis: Vec4, y_axis: Vec4, z_axis: Vec4, w_axis: Vec4) -> Self {
        Self {
            x_axis,
            y_axis,
            z_axis,
            w_axis,
        }
    }

    pub fn multiply_vec4(&self, rhs: Vec4) -> Vec4 {
        let mut res = self.x_axis.multiply_vec4(rhs.xxxx());
        res = res.add_vec4(self.y_axis.multiply_vec4(rhs.yyyy()));
        res = res.add_vec4(self.z_axis.multiply_vec4(rhs.zzzz()));
        res = res.add_vec4(self.w_axis.multiply_vec4(rhs.wwww()));
        res
    }

    pub fn multiply_mat4(&self, rhs: &Self) -> Self {
        Self::from_cols(
            self.multiply_vec4(rhs.x_axis),
            self.multiply_vec4(rhs.y_axis),
            self.multiply_vec4(rhs.z_axis),
            self.multiply_vec4(rhs.w_axis),
        )
    }

    pub fn from_rotation_x(angle: f32) -> Self {
        let (sina, cosa) = f32::sin_cos(angle.to_radians());
        Self::from_cols(
            Vec4::X,
            Vec4(0.0, cosa, sina, 0.0),
            Vec4(0.0, -sina, cosa, 0.0),
            Vec4::W,
        )
    }

    pub fn from_rotation_y(angle: f32) -> Self {
        let (sina, cosa) = f32::sin_cos(angle.to_radians());
        Self::from_cols(
            Vec4(cosa, 0.0, -sina, 0.0),
            Vec4::Y,
            Vec4(sina, 0.0, cosa, 0.0),
            Vec4::W,
        )
    }

    pub fn from_rotation_z(angle: f32) -> Self {
        let (sina, cosa) = f32::sin_cos(angle.to_radians());
        Self::from_cols(
            Vec4(cosa, sina, 0.0, 0.0),
            Vec4(-sina, cosa, 0.0, 0.0),
            Vec4::Z,
            Vec4::W,
        )
    }

    pub fn scale(scale: Vec3) -> Self {
        Self::from_cols(
            Vec4(scale.0, 0.0, 0.0, 0.0),
            Vec4(0.0, scale.1, 0.0, 0.0),
            Vec4(0.0, 0.0, scale.2, 0.0),
            Vec4::W,
        )
    }

    pub fn from_translation(translation: Vec3) -> Self {
        Self::from_cols(
            Vec4::X,
            Vec4::Y,
            Vec4::Z,
            Vec4(translation.0, translation.1, translation.2, 1.0),
        )
    }

    pub fn ortho(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        let tx = -(right + left) / (right - left);
        let ty = -(top + bottom) / (top - bottom);
        let tz = -(far + near) / (far - near);

        Mat4::from_cols(
            Vec4(2.0 / (right - left), 0.0, 0.0, 0.0),
            Vec4(0.0, 2.0 / (top - bottom), 0.0, 0.0),
            Vec4(0.0, 0.0, -2.0 / (far - near), 0.0),
            Vec4(tx, ty, tz, 1.0),
        )
    }

    pub fn perspective(fovy: f32, aspect: f32, near: f32, far: f32) -> Self {
        let tan_half_fovy = (fovy.to_radians() / 2.0).tan();
        let sx = 1.0 / (tan_half_fovy * aspect);
        let sy = 1.0 / tan_half_fovy;
        let sz = -(far + near) / (far - near);
        let tz = -(2.0 * far * near) / (far - near);

        Mat4::from_cols(
            Vec4(sx, 0.0, 0.0, 0.0),
            Vec4(0.0, sy, 0.0, 0.0),
            Vec4(0.0, 0.0, sz, -1.0),
            Vec4(0.0, 0.0, tz, 0.0),
        )
    }

    pub fn look_at(eye: Vec3, target: Vec3, up: Vec3) -> Self {
        let forward = (eye - target).normalize();
        let up = up.cross(forward).normalize();
        let right = forward.cross(up);

        let translation = Mat4::from_translation(-eye);

        Mat4::from_cols(
            Vec4(up.0, right.0, forward.0, 0.0),
            Vec4(up.1, right.1, forward.1, 0.0),
            Vec4(up.2, right.2, forward.2, 0.0),
            Vec4(0.0, 0.0, 0.0, 1.0),
        ) * translation
    }

    pub fn trace(&self) -> f32 {
        self.x_axis.0 + self.y_axis.1 + self.z_axis.2 + self.w_axis.3
    }

    pub fn look_at_rotation(forward: Vec3, up: Vec3) -> Mat4 {
        let forward = forward.normalize();
        let right = up.cross(forward).normalize();
        let up = forward.cross(right);

        Mat4::from_cols(
            Vec4(right.0, up.0, -forward.0, 0.0),
            Vec4(right.1, up.1, -forward.1, 0.0),
            Vec4(right.2, up.2, -forward.2, 0.0),
            Vec4(0.0, 0.0, 0.0, 1.0),
        )
    }
}

impl std::ops::Mul<Mat4> for Mat4 {
    type Output = Mat4;

    fn mul(self, rhs: Mat4) -> Self::Output {
        self.multiply_mat4(&rhs)
    }
}

impl std::ops::Mul<&Mat4> for &Mat4 {
    type Output = Mat4;

    fn mul(self, rhs: &Mat4) -> Self::Output {
        self.multiply_mat4(rhs)
    }
}

impl std::ops::Mul<Mat4> for &Mat4 {
    type Output = Mat4;

    fn mul(self, rhs: Mat4) -> Self::Output {
        self.multiply_mat4(&rhs)
    }
}

impl std::ops::Mul<&Mat4> for Mat4 {
    type Output = Mat4;

    fn mul(self, rhs: &Mat4) -> Self::Output {
        self.multiply_mat4(rhs)
    }
}

impl std::fmt::Display for Mat4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..4 {
            writeln!(
                f,
                "{}, {}, {}, {}",
                self.x_axis[i], self.y_axis[i], self.z_axis[i], self.w_axis[i],
            )?
        }

        Ok(())
    }
}
