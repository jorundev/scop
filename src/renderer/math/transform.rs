use super::{matrix::Mat4, quaternion::Quaternion, vec::Vec3};

#[derive(Debug, Clone)]
pub struct Transform {
    pub position: Vec3,
    pub scale: Vec3,
    pub rotation: Quaternion,
    pub origin: Vec3,
}

impl Transform {
    pub fn new(position: Vec3, scale: Vec3, rotation: Quaternion) -> Self {
        Self {
            position,
            scale,
            rotation,
            origin: Vec3(0.0, 0.0, 0.0),
        }
    }

    pub fn new_with_origin(
        position: Vec3,
        scale: Vec3,
        rotation: Quaternion,
        origin: Vec3,
    ) -> Self {
        Self {
            position,
            scale,
            rotation,
            origin,
        }
    }

    pub fn model_matrix(&self) -> Mat4 {
        let translation_matrix = Mat4::from_translation(self.position);
        let rotation_matrix = self.rotation.rotation_matrix();
        let scale_matrix = Mat4::scale(self.scale);
        let origin_matrix = Mat4::from_translation(self.origin);

        translation_matrix * rotation_matrix * scale_matrix * origin_matrix
    }

    /*pub fn look_at(&mut self, target: Vec3, up: Vec3) {
        let view_forward = (target - self.position).normalize();
        let view_up = (up - view_forward.project(up)).normalize();

        //let view_right = view_up.cross(view_forward);

        self.rotation = Quaternion::look_rotation(view_forward, up);
    }*/

    pub fn forward(&self) -> Vec3 {
        self.rotation.rotate(Vec3(0.0, 0.0, -1.0))
    }

    pub fn left(&self) -> Vec3 {
        self.rotation.rotate(Vec3(-1.0, 0.0, 0.0))
    }

    pub fn right(&self) -> Vec3 {
        self.rotation.rotate(Vec3(1.0, 0.0, 0.0))
    }

    pub fn up(&self) -> Vec3 {
        self.rotation.rotate(Vec3(0.0, 1.0, 0.0))
    }

    pub fn down(&self) -> Vec3 {
        self.rotation.rotate(Vec3(0.0, -1.0, 0.0))
    }

    pub fn rotate_around_y(&mut self, angle: f32) {
        self.rotation = self.rotation * Quaternion::from_rotation_y(angle);
    }

    pub fn rotate_around_x(&mut self, angle: f32) {
        self.rotation = self.rotation * Quaternion::from_rotation_x(angle);
    }

    pub fn rotate_around_z(&mut self, angle: f32) {
        self.rotation = self.rotation * Quaternion::from_rotation_z(angle);
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            position: Vec3(0.0, 0.0, 0.0),
            scale: Vec3(1.0, 1.0, 1.0),
            rotation: Quaternion::new(),
            origin: Vec3(0.0, 0.0, 0.0),
        }
    }
}
