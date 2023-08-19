use std::{collections::HashMap, mem::size_of};

use crate::wavefront::{self, Face, FaceAttribute};

use super::math::{
    boundingbox::BoundingBox,
    vec::{Vec3, Vec4},
};

#[derive(Clone, Debug)]
pub struct Mesh {
    pub vao: u32,
    pub vbo: u32,
    pub ebo: u32,
    pub vertex_count: u32,
    pub index_count: u32,
    pub uv_count: u32,
}

pub struct MeshData {
    pub positions: Vec<f32>,
    pub normals: Vec<f32>,
    pub colors: Vec<f32>,
    pub uvs: Vec<f32>,
    pub indices: Vec<u32>,
}

impl Mesh {
    pub fn new(data: &MeshData) -> Self {
        let mut vao = 0;
        let mut vbo = 0;
        let mut ebo = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            let mut vbo_data = vec![];

            // Layout: Position (vec3), Color (vec3), UV (vec2)
            let mut positions_iterator = data.positions.chunks(3).map(|chunk| {
                let x = chunk.get(0).unwrap_or(&0.0);
                let y = chunk.get(1).unwrap_or(&0.0);
                let z = chunk.get(2).unwrap_or(&0.0);
                Vec3(*x, *y, *z)
            });

            let mut colors_iterator = data.colors.chunks(3).map(|chunk| {
                let x = chunk.get(0).unwrap_or(&0.0);
                let y = chunk.get(1).unwrap_or(&0.0);
                let z = chunk.get(2).unwrap_or(&0.0);
                Vec3(*x, *y, *z)
            });

            let mut normals_iterator = data.normals.chunks(3).map(|chunk| {
                let x = chunk.get(0).unwrap_or(&0.0);
                let y = chunk.get(1).unwrap_or(&0.0);
                let z = chunk.get(2).unwrap_or(&0.0);
                Vec3(*x, *y, *z)
            });

            let mut uvs_iterator = data.uvs.chunks(2).map(|chunk| {
                let x = chunk.get(0).unwrap_or(&0.0);
                let y = chunk.get(1).unwrap_or(&0.0);
                (*x, *y)
            });

            loop {
                let position = positions_iterator.next();

                let position = match position {
                    Some(position) => position,
                    None => break,
                };

                let color = colors_iterator.next().unwrap_or(Vec3(1.0, 1.0, 1.0));
                let uv = uvs_iterator.next().unwrap_or((0.0, 0.0));
                let normal = normals_iterator.next().unwrap_or(Vec3(0.0, 0.0, 0.0));

                vbo_data.push(position.0);
                vbo_data.push(position.1);
                vbo_data.push(position.2);

                vbo_data.push(normal.0);
                vbo_data.push(normal.1);
                vbo_data.push(normal.2);

                vbo_data.push(color.0);
                vbo_data.push(color.1);
                vbo_data.push(color.2);

                vbo_data.push(uv.0);
                vbo_data.push(uv.1);
            }

            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vbo_data.len() * size_of::<f32>()) as _,
                vbo_data.as_ptr() as _,
                gl::STATIC_DRAW,
            );

            gl::GenBuffers(1, &mut ebo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);

            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (data.indices.len() * size_of::<u32>()) as _,
                data.indices.as_ptr() as _,
                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                (11 * size_of::<f32>()) as _,
                std::ptr::null(),
            );

            let normal_offset = (3 * std::mem::size_of::<f32>()) as _;
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                (11 * size_of::<f32>()) as _,
                normal_offset,
            );

            let color_offset = (6 * std::mem::size_of::<f32>()) as _;
            gl::VertexAttribPointer(
                2,
                3,
                gl::FLOAT,
                gl::FALSE,
                (11 * size_of::<f32>()) as _,
                color_offset,
            );

            let uv_offset = (9 * std::mem::size_of::<f32>()) as _;
            gl::VertexAttribPointer(
                3,
                2,
                gl::FLOAT,
                gl::FALSE,
                (11 * size_of::<f32>()) as _,
                uv_offset,
            );

            gl::EnableVertexAttribArray(0);
            gl::EnableVertexAttribArray(1);
            gl::EnableVertexAttribArray(2);
            gl::EnableVertexAttribArray(3);

            let vertex_count = data.positions.len() as u32;
            let index_count = data.indices.len() as u32;
            let uv_count = data.uvs.len() as u32;

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);

            gl::BindVertexArray(0);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);

            Self {
                vao,
                vbo,
                ebo,
                vertex_count,
                index_count,
                uv_count,
            }
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
        }
    }

    pub fn unbind() {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}

impl Drop for Mesh {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteBuffers(1, &self.ebo);

            gl::BindVertexArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }
    }
}

fn triangulate(attributes: &[FaceAttribute], _positions: &Vec<Vec4>) -> Vec<[FaceAttribute; 3]> {
    let mut ret = vec![];
    /*for (i, window) in attributes.windows(3).enumerate() {
        if i % 1 == 0 { continue; }

        let first = window[0];
        let second = match window.len() {
            2 | 3 => window[1],
            1 =>
        };

        let triangle = [window[0]];
    }*/

    for i in 1..(attributes.len() - 1) {
        ret.push([
            attributes[0].clone(),
            attributes[i].clone(),
            attributes[i + 1].clone(),
        ]);
    }

    ret

    /*match attributes.len() {
        3 => {
            return vec![[
                attributes[0].clone(),
                attributes[1].clone(),
                attributes[2].clone(),
            ]]
        }
        4 => {
            return vec![
                [
                    attributes[0].clone(),
                    attributes[1].clone(),
                    attributes[2].clone(),
                ]
                .clone(),
                [
                    attributes[2].clone(),
                    attributes[3].clone(),
                    attributes[0].clone(),
                ],
            ]
        }
        _ => panic!("Invalid number of vertices"),
    }*/
}

impl From<wavefront::Obj> for MeshData {
    fn from(obj: wavefront::Obj) -> Self {
        let mut positions = vec![];
        let mut uvs = vec![];
        let mut normals: Vec<f32> = vec![];
        let mut indices: Vec<u32> = vec![];

        let mut processed_attributes: HashMap<FaceAttribute, usize> = HashMap::new();

        for face in obj.faces() {
            let triangles = triangulate(&face.attributes, &obj.positions);

            for triangle in triangles {
                for attribute in triangle {
                    let index = processed_attributes.get(&attribute);

                    let index = match index {
                        Some(index) => *index,
                        None => {
                            let position = obj.positions[attribute.position_index as usize - 1];
                            let normal = attribute
                                .normal_index
                                .map(|index| obj.normals[index as usize - 1])
                                .unwrap_or(Vec3(0.0, 0.0, 0.0))
                                .normalize();
                            let uv = attribute
                                .texture_coordinate_index
                                .map(|index| obj.uvs[index as usize - 1])
                                .unwrap_or(Vec3(0.0, 0.0, 0.0));

                            positions.push(position.0);
                            positions.push(position.1);
                            positions.push(position.2);

                            normals.push(normal.0);
                            normals.push(normal.1);
                            normals.push(normal.2);

                            uvs.push(uv.0);
                            uvs.push(uv.1);

                            let index = processed_attributes.len();
                            processed_attributes.insert(attribute, index);
                            index
                        }
                    };

                    indices.push(index as u32);
                }
            }
        }

        Self {
            positions,
            indices,
            colors: vec![],
            normals,
            uvs,
        }
    }
}

impl MeshData {
    pub fn new() -> Self {
        Self {
            positions: vec![],
            indices: vec![],
            uvs: vec![],
            colors: vec![],
            normals: vec![],
        }
    }

    fn get_lowest(vertex: &Vec3, lowest: &Option<Vec3>) -> Vec3 {
        let lowest = match lowest {
            Some(lowest) => lowest,
            None => return *vertex,
        };

        Vec3(
            lowest.0.min(vertex.0),
            lowest.1.min(vertex.1),
            lowest.2.min(vertex.2),
        )
    }

    fn get_highest(vertex: &Vec3, highest: &Option<Vec3>) -> Vec3 {
        let highest = match highest {
            Some(highest) => highest,
            None => return *vertex,
        };

        Vec3(
            highest.0.max(vertex.0),
            highest.1.max(vertex.1),
            highest.2.max(vertex.2),
        )
    }

    pub fn bounding_box(&self) -> Option<BoundingBox> {
        let mut lowest: Option<Vec3> = None;
        let mut highest: Option<Vec3> = None;

        for vertex in self.positions.chunks(3) {
            let vertex = Vec3(vertex[0], vertex[1], vertex[2]);

            lowest = Some(Self::get_lowest(&vertex, &lowest));
            highest = Some(Self::get_highest(&vertex, &highest));
        }

        Some(BoundingBox::new(lowest?, highest?))
    }

	#[rustfmt::skip]
    pub fn axes() -> Self {
		let mut mesh_data = MeshData::new();

		mesh_data.positions = vec![
			0.0, 0.0, 0.0, 1000.0, 0.0, 0.0,
			0.0, 0.0, 0.0, 0.0, 1000.0, 0.0,
			0.0, 0.0, 0.0, 0.0, 0.0, 1000.0,
		];

		mesh_data.colors = vec![
			1.0, 0.0, 0.0, 1.0, 0.0, 0.0,
       		0.0, 1.0, 0.0, 0.0, 1.0, 0.0,
        	0.0, 0.0, 1.0, 0.0, 0.0, 1.0,
		];

		mesh_data.indices = vec![
			0, 1, 2,
			3, 4, 5,
			6, 7, 8,
		];


		mesh_data
	}
}
