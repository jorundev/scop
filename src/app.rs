use std::{
    io::Write,
    rc::Rc,
    time::{Duration, Instant},
};

use sdl2::{event::Event, keyboard::Keycode, video::GLContext, video::Window, Sdl, VideoSubsystem};

use crate::{
    renderer::{
        camera::Camera,
        math::{boundingbox::BoundingBox, matrix::Mat4, transform::Transform, vec::Vec3},
        mesh::{Mesh, MeshData},
        scene_object::SceneObject,
        shader::Shader,
        texture::Texture,
        Primitive, Renderer,
    },
    truevision::Targa,
    wavefront::{Obj, WavefrontObjError, WavefrontObjParseErrorDetail},
};

pub struct App {
    sdl: Sdl,
    video: VideoSubsystem,
    context: GLContext,
    window: Window,
}

struct Flags {
    rotate: bool,
    display_bounding_box: bool,
    display_axes: bool,
    cull_back_face: bool,
    user_camera_control: bool,
    display_debug_normals: bool,
    display_debug_wireframe: bool,
    display_mesh: bool,
    display_texture: bool,
    light: bool,
}

struct Objects {
    target: Option<SceneObject>,
    bounding_box: Option<SceneObject>,
    axes: SceneObject,
}

struct Meshes {
    target: Option<Rc<Mesh>>,
    bounding_box: Option<Rc<Mesh>>,
}

struct AdvancedShaders {
    normals: Shader,
    mesh: Shader,
}

struct Shaders {
    target: Shader,
    advanced: AdvancedShaders,
    bounding_box: Shader,
}

struct Keys {
    forward: bool,
    back: bool,
    left: bool,
    right: bool,
    up: bool,
    down: bool,
    translate_forward: bool,
    translate_back: bool,
    translate_left: bool,
    translate_right: bool,
    translate_up: bool,
    translate_down: bool,
}

struct State {
    camera: Camera,
    camera_distance: f32,
    running: bool,
    bounding_box: Option<BoundingBox>,
    start_time: Instant,
    rotation_accumulator: f32,
    rotating_speed: f32,
    camera_speed: f32,
    flags: Flags,
    meshes: Meshes,
    objects: Objects,
    shaders: Shaders,
    translation_speed: f32,
    relative_mouse_movement: Option<(i32, i32)>,
    keys: Keys,
    diffuse_texture: Texture,
    mix_factor: f32,
}

impl App {
    pub fn new(sdl: Sdl, video: VideoSubsystem, context: GLContext, window: Window) -> Self {
        let ret = Self {
            sdl,
            video,
            context,
            window,
        };

        unsafe {
            gl::ClearColor(0.2, 0.2, 0.2, 1.0);
            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::CULL_FACE);
            gl::CullFace(gl::BACK);
            gl::Hint(gl::LINE_SMOOTH_HINT, gl::NICEST);
            gl::Enable(gl::LINE_SMOOTH);
        }

        ret
    }

    fn set_camera_control(&mut self, state: &mut State, value: bool) {
        state.flags.user_camera_control = value;
        println!("flags.user_camera_control: {}", value);

        self.sdl
            .mouse()
            .show_cursor(!state.flags.user_camera_control);
        self.sdl
            .mouse()
            .set_relative_mouse_mode(state.flags.user_camera_control);
    }

    fn handle_keydown(&mut self, keycode: Keycode, repeat: bool, state: &mut State) {
        let name = keycode.name();

        if repeat {
            return;
        }

        match name.as_str() {
            "B" => {
                state.flags.display_bounding_box = !state.flags.display_bounding_box;
                println!(
                    "flags.display_bounding_box: {}",
                    state.flags.display_bounding_box
                );
            }
            "M" => {
                self.set_camera_control(state, false);
                print!("Path to obj file: ");
                std::io::stdout().flush().unwrap();

                let mut buffer = String::new();
                std::io::stdin().read_line(&mut buffer).unwrap();

                self.load_model(buffer.trim(), state);
            }
            "X" => {
                state.flags.display_axes = !state.flags.display_axes;
                println!("flags.display_axes: {}", state.flags.display_axes);
            }
            "P" => {
                state.flags.rotate = !state.flags.rotate;
                println!("flags.rotate: {}", state.flags.rotate);
            }
            "Return" => {
                state.rotating_speed = -state.rotating_speed;
            }
            "I" => {
                state.flags.cull_back_face = !state.flags.cull_back_face;
                println!("flags.cull_back_face: {}", state.flags.cull_back_face);

                let face = match state.flags.cull_back_face {
                    true => gl::BACK,
                    false => gl::FRONT,
                };

                unsafe {
                    gl::CullFace(face);
                }
            }
            "N" => {
                state.flags.display_debug_normals = !state.flags.display_debug_normals;
                println!(
                    "flags.display_debug_normals: {}",
                    state.flags.display_debug_normals
                );
            }
            "Z" => {
                state.flags.display_debug_wireframe = !state.flags.display_debug_wireframe;
                println!(
                    "flags.display_debug_wireframe: {}",
                    state.flags.display_debug_wireframe
                );
            }
            "K" => {
                state.flags.display_mesh = !state.flags.display_mesh;
                println!("flags.display_mesh: {}", state.flags.display_mesh);
            }
            "T" => {
                state.flags.display_texture = !state.flags.display_texture;
                println!("flags.display_texture: {}", state.flags.display_texture);
            }
            "L" => {
                state.flags.light = !state.flags.light;
                println!("flags.light: {}", state.flags.light);
            }
            "C" => {
                self.set_camera_control(state, !state.flags.user_camera_control);
            }
            "W" => state.keys.forward = true,
            "A" => state.keys.left = true,
            "S" => state.keys.back = true,
            "D" => state.keys.right = true,
            "Space" => state.keys.up = true,
            "Left Shift" => state.keys.down = true,
            "Up" => state.keys.translate_forward = true,
            "Left" => state.keys.translate_left = true,
            "Down" => state.keys.translate_back = true,
            "Right" => state.keys.translate_right = true,
            "U" => state.keys.translate_up = true,
            "J" => state.keys.translate_down = true,
            _ => {}
        }
    }

    fn handle_event(&mut self, event: Event, state: &mut State) {
        match event {
            Event::Quit { .. } => {
                state.running = false;
            }
            Event::KeyDown {
                keycode, repeat, ..
            } => {
                if let Some(keycode) = keycode {
                    self.handle_keydown(keycode, repeat, state);
                }
            }
            Event::KeyUp {
                keycode, repeat, ..
            } => {
                if repeat {
                    return;
                }

                if let Some(keycode) = keycode {
                    match keycode.name().as_str() {
                        "W" => state.keys.forward = false,
                        "A" => state.keys.left = false,
                        "S" => state.keys.back = false,
                        "D" => state.keys.right = false,
                        "Space" => state.keys.up = false,
                        "Left Shift" => state.keys.down = false,
                        "Up" => state.keys.translate_forward = false,
                        "Left" => state.keys.translate_left = false,
                        "Down" => state.keys.translate_back = false,
                        "Right" => state.keys.translate_right = false,
                        "U" => state.keys.translate_up = false,
                        "J" => state.keys.translate_down = false,
                        _ => {}
                    }
                }
            }
            Event::MouseMotion { xrel, yrel, .. } => {
                if state.flags.user_camera_control {
                    state.relative_mouse_movement = Some((xrel, yrel));
                }
            }
            _ => {}
        }
    }

    fn handle_object_transation(&mut self, state: &mut State, delta_time: f32) {
        let target = match state.objects.target {
            Some(ref mut target) => target,
            None => return,
        };

        let forward = Vec3(0.0, 0.0, -state.translation_speed) * delta_time;
        let right = Vec3(state.translation_speed, 0.0, 0.0) * delta_time;
        let up = Vec3(0.0, state.translation_speed, 0.0) * delta_time;

        let transform = &mut target.transform;

        if state.keys.translate_forward {
            transform.position = transform.position + forward;
        }
        if state.keys.translate_right {
            transform.position = transform.position + right;
        }
        if state.keys.translate_up {
            transform.position = transform.position + up;
        }
        if state.keys.translate_back {
            transform.position = transform.position - forward;
        }
        if state.keys.translate_left {
            transform.position = transform.position - right;
        }
        if state.keys.translate_down {
            transform.position = transform.position - up;
        }
    }

    unsafe fn update(&mut self, state: &mut State, delta_time: Duration) {
        let rotating_speed = state.rotating_speed;
        let camera_speed = state.camera_speed;
        let delta_time = delta_time.as_secs_f32();

        if state.flags.display_texture {
            state.mix_factor += delta_time * 5.0;
            state.mix_factor = state.mix_factor.min(1.0);
        } else {
            state.mix_factor -= delta_time * 5.0;
            state.mix_factor = state.mix_factor.max(0.0);
        }

        if state.flags.rotate {
            if let Some(ref mut target) = state.objects.target {
                target
                    .transform
                    .rotate_around_y(rotating_speed * delta_time);
            }
        }

        self.handle_object_transation(state, delta_time);

        if state.flags.user_camera_control {
            let forward = state.camera.transform.forward() * camera_speed * delta_time;
            let up: Vec3 = state.camera.transform.up() * camera_speed * delta_time;
            let right = state.camera.transform.right() * camera_speed * delta_time;

            if state.keys.forward {
                state.camera.transform.position = state.camera.transform.position + forward;
            }
            if state.keys.back {
                state.camera.transform.position = state.camera.transform.position - forward;
            }
            if state.keys.left {
                state.camera.transform.position = state.camera.transform.position - right;
            }
            if state.keys.right {
                state.camera.transform.position = state.camera.transform.position + right;
            }
            if state.keys.up {
                state.camera.transform.position = state.camera.transform.position + up;
            }
            if state.keys.down {
                state.camera.transform.position = state.camera.transform.position - up;
            }

            self.sdl.mouse().warp_mouse_in_window(
                &self.window,
                self.window.size().0 as i32 / 2,
                self.window.size().1 as i32 / 2,
            );
        }

        if let Some((xrel, yrel)) = state.relative_mouse_movement {
            state
                .camera
                .transform
                .rotate_around_y(-xrel as f32 * delta_time * 5.0);
        }

        state.camera.apply_transform();

        state.relative_mouse_movement = None;
    }

    unsafe fn render(&mut self, state: &mut State) {
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

        if let Some(ref mut scene_object) = state.objects.target {
            if state.flags.display_mesh {
                state.diffuse_texture.bind_slot(0);

                let mix_factor_location = state.shaders.target.uniform_location("mixFactor");
                let light_factor_location = state.shaders.target.uniform_location("lightFactor");

                let light_factor = state.flags.light as i32 as f32;

                state
                    .shaders
                    .target
                    .set_uniform_1f_opt(light_factor_location, light_factor);

                state
                    .shaders
                    .target
                    .set_uniform_1f_opt(mix_factor_location, state.mix_factor);

                Renderer::draw_object(
                    &scene_object,
                    &state.shaders.target,
                    &state.camera,
                    Primitive::Triangles,
                );
            }

            if state.flags.display_debug_normals {
                Renderer::draw_object(
                    &scene_object,
                    &state.shaders.advanced.normals,
                    &state.camera,
                    Primitive::Triangles,
                );
            }

            if state.flags.display_debug_wireframe {
                Renderer::draw_object(
                    &scene_object,
                    &state.shaders.advanced.mesh,
                    &state.camera,
                    Primitive::Triangles,
                );
            }

            if state.flags.display_bounding_box {
                if let Some(ref mut bbox) = state.objects.bounding_box {
                    bbox.transform = scene_object.transform.clone();
                    Renderer::draw_object(
                        bbox,
                        &state.shaders.bounding_box,
                        &state.camera,
                        Primitive::Wireframe,
                    );
                }
            }
        }

        if state.flags.display_axes {
            let axes = &state.objects.axes;
            Renderer::draw_object(
                axes,
                &state.shaders.bounding_box,
                &state.camera,
                Primitive::Wireframe,
            );
        }
    }

    fn handle_obj_error(error: WavefrontObjError) {
        match error {
            WavefrontObjError::ParseError { file, line, detail } => {
                let file = file.unwrap_or("inline".to_string());
                let detail = match detail {
                    WavefrontObjParseErrorDetail::UnknownCommand(command) => {
                        format!("Unknown command: {command}")
                    }
                    WavefrontObjParseErrorDetail::VertexParseFloatError(_)
                    | WavefrontObjParseErrorDetail::UVParseFloatError(_)
                    | WavefrontObjParseErrorDetail::NormalParseFloatError(_) => {
                        String::from("Malformed float")
                    }
                    WavefrontObjParseErrorDetail::FaceParseIntError(_) => {
                        String::from("Malformed unsigned int")
                    }
                    WavefrontObjParseErrorDetail::InvalidFaceOperand(value) => {
                        format!("Invalid index: {value}")
                    }
                    WavefrontObjParseErrorDetail::InvalidOperandCount { expected, got } => {
                        match expected {
                            (None, None) => unreachable!(),
                            (Some(a), Some(b)) => {
                                if a == b {
                                    format!("Invalid operand count. Expected {a}, got {got}")
                                } else {
                                    format!("Invalid operand count. Expected a value between {a} and {b}, got {got}")
                                }
                            }
                            (Some(a), None) => {
                                format!("Invalid operand count. Expected at least {a}, got {got}")
                            }
                            (None, Some(b)) => {
                                format!("Invalid operand count. Expected at most {b}, got {got}")
                            }
                        }
                    }
                };

                eprintln!("{file}:{line}\n\x1b[0;31merror:\x1b[0m {detail}");
            }
            _ => eprintln!("{:?}", error),
        }
    }

    fn create_bounding_box_mesh(bounding_box: Option<BoundingBox>) -> Option<Mesh> {
        let bounding_box = bounding_box?;
        let vertices = bounding_box.get_vertices();

        let mut mesh_data = MeshData::new();

        let positions: Vec<f32> = vertices
            .iter()
            .flat_map(|&vertex| vec![vertex.0, vertex.1, vertex.2])
            .collect();

        mesh_data.positions = positions;

        mesh_data.indices = vec![
            0, 1, 1, 3, 3, 2, 2, 0, // Front face
            4, 5, 5, 7, 7, 6, 6, 4, // Back face
            0, 4, 1, 5, 2, 6, 3, 7,
        ];

        let red = vec![1.0, 0.0, 0.0];

        mesh_data.colors = red
            .iter()
            .cycle()
            .take(3 * mesh_data.indices.len())
            .cloned()
            .collect();

        Some(Mesh::new(&mesh_data))
    }

    fn load_model(&mut self, path: &str, state: &mut State) {
        let obj = match Obj::from_file(path) {
            Ok(obj) => obj,
            Err(error) => {
                Self::handle_obj_error(error);
                return;
            }
        };

        println!(
            "Successfully loaded '{path}'. Total: {} vertices, {} faces",
            obj.positions.len(),
            obj.faces.len()
        );

        self.window.set_title("Scop").unwrap();

        let mesh_data = MeshData::from(obj);

        state.bounding_box = mesh_data.bounding_box();
        let mut transform = Transform::default();

        if let Some(bounding_box) = state.bounding_box {
            let center = bounding_box.center();
            transform.origin = -center;
        }

        state.meshes.target = Some(Rc::new(Mesh::new(&mesh_data)));

        if let Some(ref mesh) = state.meshes.target {
            state.objects.target = Some(SceneObject::new(mesh.clone(), transform.clone()));
        }

        state.meshes.bounding_box =
            Self::create_bounding_box_mesh(state.bounding_box).map(|m| Rc::new(m));

        let bounding_box_object = match state.meshes.bounding_box {
            Some(ref mesh) => Some(SceneObject::new(mesh.clone(), Transform::default())),
            None => None,
        };

        state.objects.bounding_box = bounding_box_object;
    }

    pub fn run(&mut self, model_path: Option<&str>) {
        let mut event_pump = self.sdl.event_pump().unwrap();

        let shader = Shader::from_file("res/shaders/phong.glsl").unwrap();
        let normal_shader_3d = Shader::from_file("res/shaders/advanced/normals.glsl").unwrap();
        let mesh_shader = Shader::from_file("res/shaders/advanced/mesh.glsl").unwrap();

        let size = self.window.size();
        let aspect = size.0 as f32 / size.1 as f32;

        let mut camera = Camera::new_perspective(100.0, 0.001, 1000.0, aspect);

        camera.transform.position = Vec3(0.0, 0.0, 5.0);
        camera.apply_transform();

        let start_time = Instant::now();

        let bounding_box_shader = Shader::from_file("res/shaders/solid.glsl").unwrap();

        let axes_mesh = Rc::new(Mesh::new(&MeshData::axes()));

        let targa = Targa::from_file("res/textures/mlp.tga").unwrap();

        let diffuse_texture = Texture::from_targa(&targa);

        let mut state = State {
            camera,
            camera_distance: 5.0,
            running: true,
            bounding_box: None,
            start_time,
            rotation_accumulator: 0.0,
            rotating_speed: 15.0,
            camera_speed: 5.0,
            flags: Flags {
                rotate: true,
                display_bounding_box: false,
                display_axes: false,
                cull_back_face: true,
                user_camera_control: false,
                display_debug_normals: false,
                display_mesh: true,
                display_debug_wireframe: false,
                display_texture: false,
                light: false,
            },
            meshes: Meshes {
                target: None,
                bounding_box: None,
            },
            objects: Objects {
                target: None,
                bounding_box: None,
                axes: SceneObject::new(axes_mesh, Transform::default()),
            },
            shaders: Shaders {
                target: shader,
                advanced: AdvancedShaders {
                    normals: normal_shader_3d,
                    mesh: mesh_shader,
                },
                bounding_box: bounding_box_shader,
            },
            relative_mouse_movement: None,
            keys: Keys {
                forward: false,
                back: false,
                left: false,
                right: false,
                up: false,
                down: false,
                translate_forward: false,
                translate_back: false,
                translate_left: false,
                translate_right: false,
                translate_up: false,
                translate_down: false,
            },
            translation_speed: 5.0,
            diffuse_texture,
            mix_factor: 0.0,
        };

        if let Some(path) = model_path {
            self.load_model(path, &mut state);
        }

        let mut last_frame_time = start_time;

        while state.running {
            for event in event_pump.poll_iter() {
                self.handle_event(event, &mut state);
            }

            let current_time = Instant::now();
            let delta_time = current_time.duration_since(last_frame_time);
            last_frame_time = current_time;

            unsafe {
                self.update(&mut state, delta_time);
                self.render(&mut state);
            }

            self.window.gl_swap_window();
        }
    }
}
