use core::ffi::c_void;

use alloc::vec::Vec;
use once_cell::sync::OnceCell;
use windows_sys::Win32::Graphics::OpenGL::wglGetProcAddress;

#[allow(dead_code)]
pub enum ShaderType {
    Fragment,
    Vertex,
    Geometry,
    Compute,
}

impl Into<i32> for ShaderType {
    fn into(self) -> i32 {
        match self {
            ShaderType::Fragment => 0x8b30,
            ShaderType::Vertex => 0x8b31,
            ShaderType::Geometry => 0x8dd9,
            ShaderType::Compute => 0x91b9,
        }
    }
}

macro_rules! glcall {
    ($t:ty, $fn:literal) => {{
        static CELL: OnceCell<$t> = OnceCell::new();
        CELL.get_or_init(|| core::mem::transmute(load(concat!($fn, "\0").as_bytes())))
    }};
}

pub struct Program {
    idx: u32,
    uniforms: Vec<(*const u8, i32)>,
}

impl Program {
    pub unsafe fn new(shader_type: ShaderType, source: &'static [u8]) -> Program {
        Program {
            idx: glcall!(unsafe extern "C" fn(i32, u32, &*const u8) -> u32, "glCreateShaderProgramv")(shader_type.into(), 1, &source.as_ptr()),
            uniforms: Vec::new(),
        }
    }

    pub unsafe fn bind(&self) {
        glcall!(unsafe extern "C" fn(u32) -> c_void, "glUseProgram")(self.idx);
    }

    pub unsafe fn set_uniform_f32(&mut self, name: &'static [u8], value: f32) {
        glcall!(unsafe extern "C" fn(u32, i32, f32) -> c_void, "glProgramUniform1f")(self.idx, self.get_uniform_location(name), value);
    }

    unsafe fn get_uniform_location(&mut self, name: &'static [u8]) -> i32 {
        if let Some(cached) = self.uniforms.iter().find(|x| x.0 == name.as_ptr()) {
            return cached.1;
        }

        let location = glcall!(unsafe extern "C" fn(u32, *const u8) -> i32, "glGetUniformLocation")(self.idx, name.as_ptr());
        self.uniforms.push((name.as_ptr(), location));
        location
    }
}

unsafe fn load(name: &'static [u8]) -> unsafe extern "system" fn() -> isize {
    wglGetProcAddress(name.as_ptr()).unwrap()
}
