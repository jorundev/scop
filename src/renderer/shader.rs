use std::{
    collections::HashMap,
    ffi::CString,
    fs::File,
    io::{self, BufRead, BufReader},
    path::PathBuf,
};

use gl::types::{GLchar, GLenum, GLint, GLsizei, GLuint};

use crate::utils::NonNegativeI32;

#[derive(Debug)]
pub struct ShaderSource {
    pub vertex_source: String,
    pub fragment_source: String,
    pub geometry_source: Option<String>,
}

#[derive(Debug)]
pub enum ShaderUniformType {
    Int,
    Uint,
    Float,
    Vec2,
    Vec3,
    Vec4,
    Mat4,
    Sampler2D,
}

impl ShaderUniformType {
    pub fn from_raw(raw_typ: GLenum) -> Self {
        return match raw_typ {
            gl::FLOAT => Self::Float,
            gl::FLOAT_VEC2 => Self::Vec2,
            gl::FLOAT_VEC3 => Self::Vec3,
            gl::FLOAT_VEC4 => Self::Vec4,
            gl::INT => Self::Int,
            gl::UNSIGNED_INT => Self::Uint,
            gl::FLOAT_MAT4 => Self::Mat4,
            gl::SAMPLER_2D => Self::Sampler2D,
            other => panic!("Unknown shader param type: {:?}", other),
        };
    }
}

#[derive(Debug)]
pub struct ShaderUniformInfo {
    name: String,
    location: Option<NonNegativeI32>,
    typ: ShaderUniformType,
}

#[derive(Debug)]
pub struct Shader {
    program: RawProgram,
    uniforms: HashMap<String, ShaderUniformInfo>,
}

#[derive(Debug)]
struct RawShader {
    raw: u32,
}

impl Drop for RawShader {
    fn drop(&mut self) {
        unsafe { gl::DeleteShader(self.raw) }
    }
}

#[derive(Debug)]
struct RawProgram {
    raw: u32,
}

impl Drop for RawProgram {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.raw) }
    }
}

#[derive(Debug)]
enum ShaderPragma {
    Shared,
    Vertex,
    Fragment,
    Geometry,
}

#[derive(Debug)]
pub enum ShaderError {
    IoError(io::Error),
    UnknownPragma(String),
    VertexCompileError(String),
    FragmentCompileError(String),
    GeometryCompileError(String),
    EntryPointError(String),
    LinkError(String),
    MissingVertexEntryPoint,
    MissingFragmentEntryPoint,
}

impl From<io::Error> for ShaderError {
    fn from(error: io::Error) -> Self {
        ShaderError::IoError(error)
    }
}

fn replace_entry_with_main(line: &str) -> Result<String, ShaderError> {
    let parts: Vec<_> = line.split_whitespace().into_iter().collect();

    if parts.len() < 3 || parts[0] != "@entry" {
        return Err(ShaderError::EntryPointError(
            "Invalid entry point".to_string(),
        ));
    }

    let function_name = parts[2].split("(").next();

    let function_name = match function_name {
        Some(name) => name,
        None => {
            return Err(ShaderError::EntryPointError(
                "Invalid entry point".to_string(),
            ))
        }
    };

    let new_function = parts[2].replacen(function_name, "main", 1);

    let mut line = format!("{} {}", parts[1], new_function);

    for part in &parts[3..] {
        line += format!("{} ", part).as_str();
    }

    Ok(line)
}

impl ShaderSource {
    pub fn from_file<P: Into<PathBuf>>(path: P) -> Result<Self, ShaderError> {
        let file = File::open(path.into())?;
        let reader = BufReader::new(file);

        let mut is_vertex_entry_point = false;
        let mut is_fragment_entry_point = false;
        let mut is_geometry_entry_point = false;

        struct UnprocessedShaderSource {
            vertex_source: String,
            fragment_source: String,
            geometry_source: String,
        }

        let mut shader_source = UnprocessedShaderSource {
            vertex_source: String::new(),
            fragment_source: String::new(),
            geometry_source: String::new(),
        };

        let mut pragma = ShaderPragma::Shared;

        for line in reader.lines() {
            let mut line = line? + "\n";

            if line.trim().len() == 0 {
                continue;
            }

            if line.starts_with("#pragma") {
                let pragma_str = line[8..].trim();

                pragma = match pragma_str {
                    "shared" => ShaderPragma::Shared,
                    "vertex" => ShaderPragma::Vertex,
                    "fragment" => ShaderPragma::Fragment,
                    "geometry" => ShaderPragma::Geometry,
                    _ => return Err(ShaderError::UnknownPragma(String::from(pragma_str))),
                };

                continue;
            } else if line.starts_with("@entry") {
                match pragma {
                    ShaderPragma::Fragment => {
                        if is_fragment_entry_point {
                            return Err(ShaderError::EntryPointError(
                                "Fragment shader entry point already defined".to_string(),
                            ));
                        }

                        is_fragment_entry_point = true;
                        line = replace_entry_with_main(&mut line)?;
                    }
                    ShaderPragma::Vertex => {
                        if is_vertex_entry_point {
                            return Err(ShaderError::EntryPointError(
                                "Vertex shader entry point already defined".to_string(),
                            ));
                        }

                        is_vertex_entry_point = true;
                        line = replace_entry_with_main(&mut line)?;
                    }
                    ShaderPragma::Geometry => {
                        if is_geometry_entry_point {
                            return Err(ShaderError::EntryPointError(
                                "Geometry shader entry point already defined".to_string(),
                            ));
                        }

                        is_geometry_entry_point = true;
                        line = replace_entry_with_main(&mut line)?;
                    }
                    ShaderPragma::Shared => {
                        return Err(ShaderError::EntryPointError(
                            "No entry point allowed for shared GLSL code".to_string(),
                        ))
                    }
                }
            }

            match pragma {
                ShaderPragma::Shared => {
                    shader_source.vertex_source += line.as_str();
                    shader_source.fragment_source += line.as_str();
                    shader_source.geometry_source += line.as_str();
                }
                ShaderPragma::Vertex => {
                    shader_source.vertex_source += line.as_str();
                }
                ShaderPragma::Fragment => {
                    shader_source.fragment_source += line.as_str();
                }
                ShaderPragma::Geometry => {
                    shader_source.geometry_source += line.as_str();
                }
            }
        }

        if !is_vertex_entry_point {
            return Err(ShaderError::MissingVertexEntryPoint);
        }
        if !is_fragment_entry_point {
            return Err(ShaderError::MissingFragmentEntryPoint);
        }

        Ok(ShaderSource {
            vertex_source: shader_source.vertex_source,
            fragment_source: shader_source.fragment_source,
            geometry_source: if is_geometry_entry_point {
                Some(shader_source.geometry_source)
            } else {
                None
            },
        })
    }
}

impl Shader {
    pub fn from_file<P: Into<PathBuf>>(path: P) -> Result<Self, ShaderError> {
        let source = ShaderSource::from_file(path)?;
        Self::from_source(&source)
    }

    pub fn bind(&self) {
        unsafe { gl::UseProgram(self.program.raw) }
    }

    pub fn unbind() {
        unsafe { gl::UseProgram(0) }
    }

    pub unsafe fn raw_program(&self) -> u32 {
        self.program.raw
    }

    pub fn set_uniform_1f(&self, location: NonNegativeI32, value: f32) {
        self.bind();
        unsafe {
            gl::Uniform1f(location.0, value);
        }
        Self::unbind();
    }

    pub fn set_uniform_1f_opt(&self, location: Option<NonNegativeI32>, value: f32) {
        if let Some(location) = location {
            self.set_uniform_1f(location, value);
        }
    }

    pub fn set_uniform_1i(&self, location: NonNegativeI32, value: i32) {
        self.bind();
        unsafe {
            gl::Uniform1i(location.0, value);
        }
        Self::unbind();
    }

    pub fn set_uniform_1i_opt(&self, location: Option<NonNegativeI32>, value: i32) {
        if let Some(location) = location {
            self.set_uniform_1i(location, value);
        }
    }

    unsafe fn get_uniform_info(program: u32) -> HashMap<String, ShaderUniformInfo> {
        let mut uniform_count: GLint = 0;

        gl::GetProgramiv(program, gl::ACTIVE_UNIFORMS, &mut uniform_count);

        if uniform_count < 0 {
            return HashMap::new();
        }

        let mut ret = HashMap::with_capacity(uniform_count as usize);

        let mut buffer: [u8; 512] = [0; 512];

        let mut length: GLsizei = 0;
        let mut typ: GLenum = 0;
        let mut size: GLint = 0;

        for i in 0..uniform_count {
            gl::GetActiveUniform(
                program,
                i as GLuint,
                511,
                &mut length,
                &mut size,
                &mut typ,
                buffer.as_mut_ptr() as _,
            );

            let buffer: Vec<u8> = (&buffer[..(length as usize)])
                .into_iter()
                .map(|u| *u)
                .collect();

            let name = String::from_utf8(buffer.clone()).unwrap();

            let mut buffer = name.as_bytes().to_vec();
            buffer.push(0);

            let location = gl::GetUniformLocation(program, buffer.as_ptr() as _);

            let location = if location >= 0 {
                Some(NonNegativeI32(location))
            } else {
                None
            };

            ret.insert(
                name.clone(),
                ShaderUniformInfo {
                    name,
                    location,
                    typ: ShaderUniformType::from_raw(typ),
                },
            );
        }

        ret
    }

    pub fn from_source(source: &ShaderSource) -> Result<Self, ShaderError> {
        let vertex_shader;
        let fragment_shader;
        let geometry_shader;

        let program;

        unsafe {
            vertex_shader = Self::compile_shader(&source.vertex_source, gl::VERTEX_SHADER)?;
            geometry_shader = match source.geometry_source {
                Some(ref source) => Some(Self::compile_shader(source, gl::GEOMETRY_SHADER)?),
                None => None,
            };
            fragment_shader = Self::compile_shader(&source.fragment_source, gl::FRAGMENT_SHADER)?;

            let mut shaders = vec![&vertex_shader, &fragment_shader];

            if let Some(ref geometry_shader) = geometry_shader {
                shaders.push(geometry_shader);
            }

            program = Self::link_shaders(&shaders[..])?;
        }

        let raw = program.raw;

        Ok(Self {
            program,
            uniforms: unsafe { Self::get_uniform_info(raw) },
        })
    }

    unsafe fn compile_shader(source: &str, typ: GLenum) -> Result<RawShader, ShaderError> {
        let shader = gl::CreateShader(typ);

        let c_str = CString::new(source.as_bytes()).unwrap();

        gl::ShaderSource(shader, 1, &c_str.as_ptr(), std::ptr::null());
        gl::CompileShader(shader);

        let mut success = gl::FALSE as GLint;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);

        if success != gl::TRUE as GLint {
            let mut info_log_length: GLint = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut info_log_length);

            let mut log: Vec<u8> = vec![0; info_log_length as usize];
            let mut length_written: GLint = 0;

            gl::GetShaderInfoLog(
                shader,
                info_log_length,
                &mut length_written,
                log.as_mut_ptr() as *mut GLchar,
            );

            let log: String = String::from_utf8_lossy(&log[0..length_written as usize]).into();

            gl::DeleteShader(shader);

            return match typ {
                gl::VERTEX_SHADER => Err(ShaderError::VertexCompileError(log)),
                gl::FRAGMENT_SHADER => Err(ShaderError::FragmentCompileError(log)),
                gl::GEOMETRY_SHADER => Err(ShaderError::GeometryCompileError(log)),
                _ => unreachable!("Invalid shader type: {:?}", typ),
            };
        }

        Ok(RawShader { raw: shader })
    }

    unsafe fn link_shaders(shaders: &[&RawShader]) -> Result<RawProgram, ShaderError> {
        let program = gl::CreateProgram();

        for shader in shaders.iter() {
            gl::AttachShader(program, shader.raw);
        }

        gl::LinkProgram(program);

        let mut success = gl::FALSE as GLint;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);

        if success != gl::TRUE as i32 {
            let mut info_log_length: GLint = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut info_log_length);

            let mut log: Vec<u8> = vec![0; info_log_length as usize];
            let mut length_written: GLint = 0;

            gl::GetProgramInfoLog(
                program,
                info_log_length,
                &mut length_written,
                log.as_mut_ptr() as *mut GLchar,
            );

            let log: String = String::from_utf8_lossy(&log[0..length_written as usize]).into();

            gl::DeleteProgram(program);

            return Err(ShaderError::LinkError(log));
        }

        Ok(RawProgram { raw: program })
    }

    pub fn uniform_location(&self, name: &str) -> Option<NonNegativeI32> {
        Some(self.uniforms.get(name)?.location?)
    }
}
