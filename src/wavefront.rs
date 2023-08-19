use std::ffi::OsString;
use std::io::Read;

use std::num::{ParseFloatError, ParseIntError};
use std::{fs::File, io, path::PathBuf};

use crate::renderer::math::vec::{Vec3, Vec4};

#[derive(Debug)]
pub enum WavefrontObjParseErrorDetail {
    UnknownCommand(String),
    InvalidOperandCount {
        expected: (Option<usize>, Option<usize>),
        got: usize,
    },
    VertexParseFloatError(ParseFloatError),
    UVParseFloatError(ParseFloatError),
    NormalParseFloatError(ParseFloatError),
    FaceParseIntError(ParseIntError),
    InvalidFaceOperand(u32),
}

#[derive(Debug)]
pub enum WavefrontObjError {
    IoError(io::Error),
    PathError(OsString),
    ParseError {
        file: Option<String>,
        line: usize,
        detail: WavefrontObjParseErrorDetail,
    },
}

impl From<io::Error> for WavefrontObjError {
    fn from(error: io::Error) -> Self {
        WavefrontObjError::IoError(error)
    }
}

impl From<OsString> for WavefrontObjError {
    fn from(error: OsString) -> Self {
        WavefrontObjError::PathError(error)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FaceAttribute {
    pub position_index: u32,
    pub texture_coordinate_index: Option<u32>,
    pub normal_index: Option<u32>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Face {
    pub attributes: Vec<FaceAttribute>,
}

impl Face {
    pub fn attributes(&self) -> &[FaceAttribute] {
        &self.attributes
    }
}

#[derive(Debug)]
pub struct Obj {
    pub positions: Vec<Vec4>,
    pub uvs: Vec<Vec3>,
    pub normals: Vec<Vec3>,
    pub faces: Vec<Face>,
}

impl Obj {
    pub fn from_file<P: Into<PathBuf>>(path: P) -> Result<Self, WavefrontObjError> {
        let path: PathBuf = path.into();
        let path_str = path.clone().into_os_string().into_string()?;
        let mut file = File::open(path)?;
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();

        Self::from_string(&data, Some(&path_str))
    }

    pub fn vertices(&self) -> &[Vec4] {
        &self.positions
    }

    pub fn faces(&self) -> &[Face] {
        &self.faces
    }

    fn check_operand_length(
        min: usize,
        mut max: usize,
        val: usize,
    ) -> Option<WavefrontObjParseErrorDetail> {
        if max == 0 {
            max = usize::MAX;
        }
        if val < min || val > max {
            let min = if min == 0 { None } else { Some(min) };
            let max = if max == 0 { None } else { Some(max) };
            return Some(WavefrontObjParseErrorDetail::InvalidOperandCount {
                expected: (min, max),
                got: val,
            });
        }

        None
    }

    fn parse_floats_from_line(operands: &[&str]) -> Result<Vec<f32>, ParseFloatError> {
        let mut floats = vec![];

        for operand in operands {
            floats.push(operand.parse::<f32>()?);
        }

        Ok(floats)
    }

    fn parse_face_from_line(operands: &[&str]) -> Result<Vec<FaceAttribute>, ParseIntError> {
        let mut ret = vec![];

        for operand in operands {
            let parts: Vec<_> = operand.split("/").collect();

            let mut face_data = FaceAttribute {
                position_index: 0,
                texture_coordinate_index: None,
                normal_index: None,
            };

            face_data.position_index = parts[0].parse::<u32>()?;
            face_data.texture_coordinate_index = match parts.get(1) {
                Some(str) => {
                    if *str == "" {
                        None
                    } else {
                        Some(str.parse::<u32>()?)
                    }
                }
                None => None,
            };
            face_data.normal_index = match parts.get(2) {
                Some(str) => {
                    if *str == "" {
                        None
                    } else {
                        Some(str.parse::<u32>()?)
                    }
                }
                None => None,
            };

            ret.push(face_data);
        }

        Ok(ret)
    }

    fn parse_error(
        file_name: Option<&str>,
        line: usize,
        detail: WavefrontObjParseErrorDetail,
    ) -> WavefrontObjError {
        WavefrontObjError::ParseError {
            file: file_name.map(|s| s.to_string()),
            line: line + 1,
            detail,
        }
    }

    fn handle_positions_line(
        file_name: Option<&str>,
        line: usize,
        operands: &[&str],
        positions: &mut Vec<Vec4>,
    ) -> Result<(), WavefrontObjError> {
        if let Some(detail) = Self::check_operand_length(3, 4, operands.len()) {
            return Err(Self::parse_error(file_name, line, detail));
        }

        let coordinates = match Self::parse_floats_from_line(operands) {
            Ok(coords) => coords,
            Err(e) => {
                return Err(Self::parse_error(
                    file_name,
                    line,
                    WavefrontObjParseErrorDetail::VertexParseFloatError(e),
                ));
            }
        };

        let x = coordinates.get(0).unwrap_or(&0.0);
        let y = coordinates.get(1).unwrap_or(&0.0);
        let z = coordinates.get(2).unwrap_or(&0.0);
        let w = coordinates.get(3).unwrap_or(&1.0);
        positions.push(Vec4(*x, *y, *z, *w));

        Ok(())
    }

    fn handle_face_line(
        file_name: Option<&str>,
        line: usize,
        operands: &[&str],
        faces: &mut Vec<Face>,
    ) -> Result<(), WavefrontObjError> {
        if let Some(detail) = Self::check_operand_length(3, 0, operands.len()) {
            return Err(Self::parse_error(file_name, line, detail));
        }

        let face_data = Self::parse_face_from_line(operands);

        let face_data = match face_data {
            Ok(data) => data,
            Err(e) => {
                return Err(Self::parse_error(
                    file_name,
                    line,
                    WavefrontObjParseErrorDetail::FaceParseIntError(e),
                ));
            }
        };

        let positions: Vec<_> = face_data.iter().map(|data| data.position_index).collect();

        if positions.contains(&0) {
            return Err(Self::parse_error(
                file_name,
                line,
                WavefrontObjParseErrorDetail::InvalidFaceOperand(0),
            ));
        }
        faces.push(Face {
            attributes: face_data,
        });

        Ok(())
    }

    fn handle_uv_line(
        file_name: Option<&str>,
        line: usize,
        operands: &[&str],
        uvs: &mut Vec<Vec3>,
    ) -> Result<(), WavefrontObjError> {
        if let Some(detail) = Self::check_operand_length(1, 3, operands.len()) {
            return Err(Self::parse_error(file_name, line, detail));
        }

        let uv_floats = match Self::parse_floats_from_line(operands) {
            Ok(uv) => uv,
            Err(e) => {
                return Err(Self::parse_error(
                    file_name,
                    line,
                    WavefrontObjParseErrorDetail::UVParseFloatError(e),
                ));
            }
        };

        let u = uv_floats.get(0).unwrap_or(&0.0);
        let v = uv_floats.get(1).unwrap_or(&0.0);
        let w = uv_floats.get(2).unwrap_or(&0.0);

        uvs.push(Vec3(*u, *v, *w));

        Ok(())
    }

    fn handle_normal_line(
        file_name: Option<&str>,
        line: usize,
        operands: &[&str],
        normals: &mut Vec<Vec3>,
    ) -> Result<(), WavefrontObjError> {
        if let Some(detail) = Self::check_operand_length(3, 3, operands.len()) {
            return Err(Self::parse_error(file_name, line, detail));
        }

        let normal_floats = match Self::parse_floats_from_line(operands) {
            Ok(uv) => uv,
            Err(e) => {
                return Err(Self::parse_error(
                    file_name,
                    line,
                    WavefrontObjParseErrorDetail::NormalParseFloatError(e),
                ));
            }
        };

        let x = normal_floats.get(0).unwrap_or(&0.0);
        let y = normal_floats.get(1).unwrap_or(&0.0);
        let z = normal_floats.get(2).unwrap_or(&0.0);

        normals.push(Vec3(*x, *y, *z));

        Ok(())
    }

    pub fn from_string(data: &str, file_name: Option<&str>) -> Result<Self, WavefrontObjError> {
        let lines = data.lines();

        // Pre-allocating so we feel less guilty
        let mut positions = Vec::with_capacity(4096);
        let mut uvs = Vec::with_capacity(4096);
        let mut normals = Vec::with_capacity(4096);
        let mut faces = Vec::with_capacity(4096);

        for (i, line) in lines.enumerate() {
            let line = line.trim();

            if line.len() == 0 || line.starts_with("#") {
                continue;
            }

            let elements = line.split_whitespace().collect::<Vec<_>>();

            let operands = &elements[1..];

            match elements[0] {
                "v" => {
                    Self::handle_positions_line(file_name, i, operands, &mut positions)?;
                }
                "f" => {
                    Self::handle_face_line(file_name, i, operands, &mut faces)?;
                }
                "mtllib" => {
                    // TODO
                }
                "usemtl" => {
                    // TODO
                }
                "s" => {
                    // TODO
                }
                "g" => {
                    // TODO
                }
                "o" => {
                    // TODO
                }
                "vt" => {
                    Self::handle_uv_line(file_name, i, operands, &mut uvs)?;
                }
                "vn" => {
                    Self::handle_normal_line(file_name, i, operands, &mut normals)?;
                }
                _ => {
                    return Err(Self::parse_error(
                        file_name,
                        i,
                        WavefrontObjParseErrorDetail::UnknownCommand(elements[0].to_string()),
                    ));
                }
            }
        }

        Ok(Obj {
            positions,
            faces,
            normals,
            uvs,
        })
    }
}
