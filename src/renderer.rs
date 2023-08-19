use self::{camera::Camera, mesh::Mesh, scene_object::SceneObject, shader::Shader};

pub mod camera;
pub mod math;
pub mod mesh;
pub mod scene_object;
pub mod shader;
pub mod texture;

pub struct Renderer;

#[derive(Debug, Clone, Copy)]
pub enum Primitive {
    Triangles,
    Wireframe,
}

impl Renderer {
    pub fn draw_object(
        object: &SceneObject,
        shader: &Shader,
        camera: &Camera,
        primitive: Primitive,
    ) {
        shader.bind();
        object.mesh().bind();

        let model_matrix = object.transform.model_matrix();

        let mvp = camera.view_projection_matrix() * model_matrix;

        unsafe {
            let mvp_location = shader.uniform_location("mvp");
            let model_location = shader.uniform_location("modelMatrix");
            let diffuse_location = shader.uniform_location("diffuseTex");

            if let Some(location) = mvp_location {
                gl::UniformMatrix4fv(location.0, 1, gl::FALSE, &mvp as *const _ as _);
            }

            if let Some(location) = model_location {
                gl::UniformMatrix4fv(location.0, 1, gl::FALSE, &model_matrix as *const _ as _);
            }

            if let Some(location) = diffuse_location {
                gl::Uniform1i(location.0, 0);
            }

            let mode = match primitive {
                Primitive::Triangles => gl::TRIANGLES,
                Primitive::Wireframe => gl::LINES,
            };

            gl::DrawElements(
                mode,
                object.mesh().index_count as i32,
                gl::UNSIGNED_INT,
                0 as _,
            );
        }

        Mesh::unbind();
        Shader::unbind();
    }
}
