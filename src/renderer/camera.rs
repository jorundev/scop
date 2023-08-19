use super::math::{matrix::Mat4, transform::Transform};

pub enum Projection {
    Perspective {
        fov: f32,
        near: f32,
        far: f32,
        aspect: f32,
    },
    Orthographic {
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
    },
}

pub struct Camera {
    pub transform: Transform,
    pub projection: Projection,
    view_matrix: Mat4,
    projection_matrix: Mat4,
    view_projection_matrix: Mat4,
}

impl Camera {
    pub fn new_perspective(fov: f32, near: f32, far: f32, aspect: f32) -> Self {
        let mut camera = Self {
            transform: Transform::default(),
            projection: Projection::Perspective {
                fov,
                near,
                far,
                aspect,
            },
            view_matrix: Mat4::IDENTITY,
            projection_matrix: Mat4::IDENTITY,
            view_projection_matrix: Mat4::IDENTITY,
        };

        camera.apply_transform();
        camera
    }

    pub fn new_perspective_at(
        transform: Transform,
        fov: f32,
        near: f32,
        far: f32,
        aspect: f32,
    ) -> Self {
        let mut camera = Self {
            transform,
            projection: Projection::Perspective {
                fov,
                near,
                far,
                aspect,
            },
            view_matrix: Mat4::IDENTITY,
            projection_matrix: Mat4::IDENTITY,
            view_projection_matrix: Mat4::IDENTITY,
        };

        camera.apply_transform();
        camera
    }

    pub fn create_view_matrix(&self) -> Mat4 {
        let translation_matrix = Mat4::from_translation(-self.transform.position);

        let rotation_matrix = self.transform.rotation.rotation_matrix();

        rotation_matrix * translation_matrix
    }

    fn create_projection_matrix(&self) -> Mat4 {
        match self.projection {
            Projection::Perspective {
                fov,
                near,
                far,
                aspect,
            } => Mat4::perspective(fov, aspect, near, far),
            Projection::Orthographic {
                left,
                right,
                bottom,
                top,
                near,
                far,
            } => Mat4::ortho(left, right, bottom, top, near, far),
        }
    }

    fn projection_matrix(&self) -> Mat4 {
        self.projection_matrix
    }

    pub fn view_projection_matrix(&self) -> Mat4 {
        self.view_projection_matrix
    }

    pub fn view_matrix(&self) -> Mat4 {
        self.view_matrix
    }

    pub fn apply_transform(&mut self) {
        self.view_matrix = self.create_view_matrix();
        self.projection_matrix = self.create_projection_matrix();
        self.view_projection_matrix = self.projection_matrix * self.view_matrix
    }
}
