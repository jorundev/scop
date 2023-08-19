use std::rc::Rc;

use super::{math::transform::Transform, mesh::Mesh};

pub struct SceneObject {
    mesh: Rc<Mesh>,
    pub transform: Transform,
}

impl SceneObject {
    pub fn new(mesh: Rc<Mesh>, transform: Transform) -> Self {
        Self { mesh, transform }
    }

    pub fn mesh(&self) -> &Mesh {
        self.mesh.as_ref()
    }
}
