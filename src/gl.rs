use core::ffi::c_void;

use once_cell::sync::OnceCell;
use windows_sys::Win32::Graphics::OpenGL::wglGetProcAddress;

pub type ProgramIdx = i32;

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
            ShaderType::Compute => 	0x91b9,
        }
    }
}

pub unsafe fn create_shader_program(shader_type: ShaderType, source: &'static str) -> ProgramIdx {
    static CELL: OnceCell<unsafe extern "C" fn(i32, u32, &*const u8) -> i32> = OnceCell::new();
    CELL.get_or_init(|| core::mem::transmute(load(b"glCreateShaderProgramv\0")))(shader_type.into(), 1, &source.as_ptr())
}

pub unsafe fn use_program(program: ProgramIdx) {
    static CELL: OnceCell<unsafe extern "C" fn(i32) -> c_void> = OnceCell::new();
    CELL.get_or_init(|| core::mem::transmute(load(b"glUseProgram\0")))(program);
}

pub unsafe fn set_uniform(location: i32, value: f32) {
    static CELL: OnceCell<unsafe extern "C" fn(i32, f32) -> c_void> = OnceCell::new();
    CELL.get_or_init(|| core::mem::transmute(load(b"glUniform1f\0")))(location, value);
}

unsafe fn load(name: &'static [u8]) -> unsafe extern "system" fn() -> isize {
    wglGetProcAddress(name.as_ptr()).unwrap()
}
