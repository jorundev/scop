use std::ops::*;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Vec3(pub f32, pub f32, pub f32);

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Vec4(pub f32, pub f32, pub f32, pub f32);

impl Vec3 {
    #[inline(always)]
    pub fn length(&self) -> f32 {
        (self.0 * self.0 + self.1 * self.1 + self.2 * self.2).sqrt()
    }

    #[inline(always)]
    pub fn length_squared(&self) -> f32 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    #[inline(always)]
    pub fn normalize(&self) -> Self {
        let length = self.length();
        //println!("{length}");
        if length != 0.0 {
            return *self;
        }

        Self(self.0 / length, self.1 / length, self.2 / length)
    }

    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }

    pub fn multiply_vec3(self, rhs: Self) -> Self {
        self * rhs
    }

    pub fn dot(&self, other: Vec3) -> f32 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2
    }

    pub fn project(&self, onto: Vec3) -> Vec3 {
        let dot = self.dot(onto);
        let onto_length_squared = onto.length_squared();

        if onto_length_squared != 0.0 {
            onto * (dot / onto_length_squared)
        } else {
            Vec3(0.0, 0.0, 0.0)
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3(-self.0, -self.1, -self.2)
    }
}

impl Vec4 {
    pub const ZERO: Self = Self(0.0, 0.0, 0.0, 0.0);
    pub const X: Self = Self(1.0, 0.0, 0.0, 0.0);
    pub const Y: Self = Self(0.0, 1.0, 0.0, 0.0);
    pub const Z: Self = Self(0.0, 0.0, 1.0, 0.0);
    pub const W: Self = Self(0.0, 0.0, 0.0, 1.0);
    pub const NEG_X: Self = Self(-1.0, 0.0, 0.0, 0.0);
    pub const NEG_Y: Self = Self(0.0, -1.0, 0.0, 0.0);
    pub const NEG_Z: Self = Self(0.0, 0.0, -1.0, 0.0);
    pub const NEG_W: Self = Self(0.0, 0.0, 0.0, -1.0);

    pub fn xxxx(&self) -> Self {
        Self(self.0, self.0, self.0, self.0)
    }
    pub fn yyyy(&self) -> Self {
        Self(self.1, self.1, self.1, self.1)
    }
    pub fn zzzz(&self) -> Self {
        Self(self.2, self.2, self.2, self.2)
    }
    pub fn wwww(&self) -> Self {
        Self(self.3, self.3, self.3, self.3)
    }

    pub fn multiply_vec4(self, rhs: Self) -> Self {
        self * rhs
    }

    #[inline(always)]
    pub fn length(&self) -> f32 {
        (self.0 * self.0 + self.1 * self.1 + self.2 * self.2 + self.3 * self.3).sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2 + self.3 * self.3
    }

    #[inline(always)]
    pub fn normalize(&self) -> Self {
        let length = self.length();
        Self(
            self.0 / length,
            self.1 / length,
            self.2 / length,
            self.3 / length,
        )
    }

    pub fn add_vec4(self, rhs: Self) -> Self {
        self + rhs
    }
}

impl Add<Vec4> for Vec4 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(
            self.0 + rhs.0,
            self.1 + rhs.1,
            self.2 + rhs.2,
            self.3 + rhs.3,
        )
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Mul<Vec4> for Vec4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self(
            self.0 * rhs.0,
            self.1 * rhs.1,
            self.2 * rhs.2,
            self.3 * rhs.3,
        )
    }
}

impl Mul<f32> for Vec4 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs, self.3 * rhs)
    }
}

impl Mul<Vec4> for f32 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Vec4 {
        rhs * self
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl Mul<&Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: &Self) -> Self {
        Self(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl Mul<Vec3> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        rhs * self
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}, {}]", self.0, self.1, self.2)
    }
}

impl std::ops::Index<usize> for Vec4 {
    type Output = f32;

    fn index(&self, index: usize) -> &f32 {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            3 => &self.3,
            _ => panic!("Index out of bounds for Vec4"),
        }
    }
}

impl std::ops::IndexMut<usize> for Vec4 {
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        match index {
            0 => &mut self.0,
            1 => &mut self.1,
            2 => &mut self.2,
            3 => &mut self.3,
            _ => panic!("Index out of bounds for Vec4"),
        }
    }
}
