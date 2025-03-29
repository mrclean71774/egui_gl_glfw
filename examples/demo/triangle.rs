// Draws a triangle with a gradient texture
// based on the example from:
// https://github.com/brendanzab/gl-rs/blob/master/gl/examples/triangle.rs

use egui_gl_glfw::gl;
use egui_gl_glfw::gl::types::*;
use std::{mem, ptr, str};

use std::ffi::CString;

#[allow(unconditional_panic, clippy::out_of_bounds_indexing)]
const fn illegal_null_in_string() {
    [][0]
}

#[doc(hidden)]
pub const fn validate_cstr_contents(bytes: &[u8]) {
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'\0' {
            illegal_null_in_string();
        }
        i += 1;
    }
}

macro_rules! cstr {
    ( $s:literal ) => {{
        validate_cstr_contents($s.as_bytes());
        unsafe { std::mem::transmute::<&str, &std::ffi::CStr>(concat!($s, "\0")) }
    }};
}

fn compile_shader(src: &str, ty: GLenum) -> GLuint {
    let shader = unsafe { gl::CreateShader(ty) };

    let c_str = CString::new(src.as_bytes()).unwrap();
    unsafe {
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), core::ptr::null());
        gl::CompileShader(shader);
    }

    let mut status = gl::FALSE as GLint;
    unsafe {
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);
    }

    if status != (gl::TRUE as GLint) {
        let mut len = 0;
        unsafe {
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
        }

        let mut buf = vec![0; len as usize];

        unsafe {
            gl::GetShaderInfoLog(
                shader,
                len,
                core::ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );
        }

        panic!(
            "{}",
            core::str::from_utf8(&buf).expect("ShaderInfoLog not valid utf8")
        );
    }

    shader
}

fn link_program(vs: GLuint, fs: GLuint) -> GLuint {
    let program = unsafe { gl::CreateProgram() };

    unsafe {
        gl::AttachShader(program, vs);
        gl::AttachShader(program, fs);
        gl::LinkProgram(program);
    }

    let mut status = gl::FALSE as GLint;
    unsafe {
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);
    }

    if status != (gl::TRUE as GLint) {
        let mut len: GLint = 0;
        unsafe {
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
        }

        let mut buf = vec![0; len as usize];

        unsafe {
            gl::GetProgramInfoLog(
                program,
                len,
                core::ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );
        }

        panic!(
            "{}",
            core::str::from_utf8(&buf).expect("ProgramInfoLog not valid utf8")
        );
    }

    program
}

fn generate_texture() -> Vec<u8> {
    let mut texture = Vec::with_capacity(256 * 256 * 256);
    for y in 0..=255 {
        for x in 0..=255 {
            texture.append(&mut vec![x, y, 0]);
        }
    }
    texture
}

const VS_SRC: &str = "
#version 150
in vec2 position;
in vec2 tex;

out vec2 tex_coord;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    tex_coord = tex;
}";

const FS_SRC: &str = "
#version 150
out vec4 out_color;

in vec2 tex_coord;

uniform sampler2D gradient;

void main() {
    out_color = texture(gradient, tex_coord);
}";

static VERTEX_DATA: [GLfloat; 12] = [
    -0.5, -0.5, 0.0, 0.0, //
    0.5, -0.5, 1.0, 0.0, //
    0.0, 0.5, 0.5, 1.0, //
];

pub struct Triangle {
    pub vs: GLuint,
    pub fs: GLuint,
    pub program: GLuint,
    pub vao: GLuint,
    pub vbo: GLuint,
    pub tex: GLuint,
}

impl Triangle {
    pub fn new() -> Self {
        let vs = compile_shader(VS_SRC, gl::VERTEX_SHADER);
        let fs = compile_shader(FS_SRC, gl::FRAGMENT_SHADER);
        let program = link_program(vs, fs);

        let mut vao = 0;
        let mut vbo = 0;
        let mut tex = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenTextures(1, &mut tex);
        }

        Triangle {
            vs,
            fs,
            program,
            vao,
            vbo,
            tex,
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (VERTEX_DATA.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                &VERTEX_DATA[0] as *const f32 as *const std::ffi::c_void,
                gl::STATIC_DRAW,
            );

            gl::BindTexture(gl::TEXTURE_2D, self.tex);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as i32,
                256,
                256,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                generate_texture().as_ptr() as *const std::ffi::c_void,
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        }

        unsafe {
            gl::UseProgram(self.program);
        }

        let c_out_color = cstr!("out_color");
        unsafe {
            gl::BindFragDataLocation(self.program, 0, c_out_color.as_ptr());
        }

        let c_position = cstr!("position");
        let pos_attr = unsafe { gl::GetAttribLocation(self.program, c_position.as_ptr()) };
        unsafe {
            gl::EnableVertexAttribArray(pos_attr as GLuint);
            gl::VertexAttribPointer(
                pos_attr as GLuint,
                2,
                gl::FLOAT,
                gl::FALSE as GLboolean,
                (4 * size_of::<GLfloat>()) as GLint,
                ptr::null(),
            );
        }
        let c_tex = cstr!("tex");
        let tex_attr = unsafe { gl::GetAttribLocation(self.program, c_tex.as_ptr()) };
        unsafe {
            gl::EnableVertexAttribArray(tex_attr as GLuint);
            gl::VertexAttribPointer(
                tex_attr as GLuint,
                2,
                gl::FLOAT,
                gl::FALSE as GLboolean,
                (4 * size_of::<GLfloat>()) as GLint,
                (2 * size_of::<GLfloat>()) as *const std::ffi::c_void,
            );
        }

        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }
}

impl Drop for Triangle {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.program);
            gl::DeleteShader(self.fs);
            gl::DeleteShader(self.vs);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteVertexArrays(1, &self.vao);
        }
    }
}
