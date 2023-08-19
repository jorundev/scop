use super::vec::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct BoundingBox {
    min_point: Vec3,
    max_point: Vec3,
}

impl BoundingBox {
    pub fn new(min_point: Vec3, max_point: Vec3) -> Self {
        BoundingBox {
            min_point,
            max_point,
        }
    }

    pub fn get_vertices(&self) -> [Vec3; 8] {
        [
            Vec3(self.min_point.0, self.min_point.1, self.min_point.2),
            Vec3(self.max_point.0, self.min_point.1, self.min_point.2),
            Vec3(self.min_point.0, self.max_point.1, self.min_point.2),
            Vec3(self.max_point.0, self.max_point.1, self.min_point.2),
            Vec3(self.min_point.0, self.min_point.1, self.max_point.2),
            Vec3(self.max_point.0, self.min_point.1, self.max_point.2),
            Vec3(self.min_point.0, self.max_point.1, self.max_point.2),
            Vec3(self.max_point.0, self.max_point.1, self.max_point.2),
        ]
    }

    pub fn contains(&self, point: Vec3) -> bool {
        point.0 >= self.min_point.0
            && point.0 <= self.max_point.0
            && point.1 >= self.min_point.1
            && point.1 <= self.max_point.1
            && point.2 >= self.min_point.2
            && point.2 <= self.max_point.2
    }

    pub fn center(&self) -> Vec3 {
        Vec3(
            (self.min_point.0 + self.max_point.0) * 0.5,
            (self.min_point.1 + self.max_point.1) * 0.5,
            (self.min_point.2 + self.max_point.2) * 0.5,
        )
    }
}
