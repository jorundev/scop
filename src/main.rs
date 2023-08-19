#![feature(rustc_attrs)]

mod app;
mod commands;

pub mod opengl;
pub mod renderer;
pub mod truevision;
pub mod utils;
pub mod wavefront;

use std::ffi::CStr;

use app::App;
use sdl2::video::GLProfile;
use truevision::Targa;

fn display_driver_info() {
    let to_str = |raw: *const u8| {
        let cstr = unsafe { CStr::from_ptr(raw as *const i8) };
        cstr.to_str().expect("Invalid UTF-8")
    };

    unsafe {
        let vendor = to_str(gl::GetString(gl::VENDOR));
        let renderer = to_str(gl::GetString(gl::RENDERER));
        let version = to_str(gl::GetString(gl::VERSION));

        println!("OpenGL info:\n  Vendor: {vendor}\n  Renderer:  {renderer}\n  Version: {version}");
    }
}

fn main() {
    let args = std::env::args();
    let size = (1280, 800);
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();

    let args: Vec<_> = args.collect();

    let model_path = match args.len() {
        1 => None,
        2 => Some(args[1].as_str()),
        _ => panic!("Invalid number of arguments"),
    };

    let attributes = video.gl_attr();

    attributes.set_context_profile(GLProfile::Core);
    attributes.set_context_version(4, 1);
    attributes.set_context_profile(GLProfile::Core);
    attributes.set_depth_size(24);
    attributes.set_stencil_size(8);
    attributes.set_double_buffer(true);

    let window = video
        .window("Scop (no model)", size.0, size.1)
        .allow_highdpi()
        .opengl()
        .build()
        .unwrap();

    let context = window.gl_create_context().unwrap();
    gl::load_with(|s| video.gl_get_proc_address(s) as *const _);

    video.gl_set_swap_interval(1).unwrap();
    display_driver_info();

    let mut app = App::new(sdl, video, context, window);

    app.run(model_path);
}
