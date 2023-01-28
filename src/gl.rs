use core::ffi::c_void;

use once_cell::sync::OnceCell;
use windows_sys::Win32::Graphics::OpenGL::wglGetProcAddress;

pub unsafe fn set_shader(source: &'static str) {
    static CELL: OnceCell<(
        unsafe extern "C" fn(i32, u32, &*const u8) -> i32,
        unsafe extern "C" fn(i32) -> c_void,
    )> = OnceCell::new();
    let gl_functions = CELL.get_or_init(|| {
        (
            core::mem::transmute(load(b"glCreateShaderProgramv\0")),
            core::mem::transmute(load(b"glUseProgram\0")),
        )
    });

    gl_functions.1(gl_functions.0(0x8B30, 1, &source.as_ptr()));
}

pub unsafe fn set_uniform(location: i32, value: f32) {
    static CELL: OnceCell<unsafe extern "C" fn(i32, f32) -> c_void> = OnceCell::new();
    CELL.get_or_init(|| core::mem::transmute(load(b"glUniform1f\0")))(location, value);
}

unsafe fn load(name: &'static [u8]) -> unsafe extern "system" fn() -> isize {
    wglGetProcAddress(name.as_ptr()).unwrap()
}
