#![no_std]
#![no_main]
#![windows_subsystem = "windows"]

#[link(name = "libcmt")]
extern "C" {}
#[link(name = "ucrt")]
extern "C" {}
#[link(name = "uuid")]
extern "C" {}
#[link(name = "vcruntime")]
extern "C" {}

use core::{ffi::c_void, mem};

use wavesabre_rs::device::{Device, DeviceId};
use windows_sys::Win32::{
    Graphics::{
        Gdi::{
            ChangeDisplaySettingsA, GetDC, CDS_FULLSCREEN, DEVMODEA, DM_BITSPERPEL, DM_PELSHEIGHT,
            DM_PELSWIDTH, HDC,
        },
        OpenGL::{
            glRects, wglCreateContext, wglMakeCurrent, ChoosePixelFormat, SetPixelFormat,
            SwapBuffers, PFD_DOUBLEBUFFER, PFD_SUPPORT_OPENGL, PIXELFORMATDESCRIPTOR,
        },
    },
    System::Threading::ExitProcess,
    UI::{
        Input::KeyboardAndMouse::{GetAsyncKeyState, VK_ESCAPE},
        WindowsAndMessaging::{CreateWindowExA, ShowCursor, WS_MAXIMIZE, WS_POPUP, WS_VISIBLE},
    },
};

mod critical;
mod gl;
mod glsl;
mod time;

static SONG_BLOB: &'static [u8] = include_bytes!("song.bin");

unsafe fn enter_fullscreen() {
    let mut mode: DEVMODEA = mem::zeroed();
    mode.dmSize = mem::size_of::<DEVMODEA>() as u16;
    mode.dmPelsWidth = 1920;
    mode.dmPelsHeight = 1080;
    mode.dmBitsPerPel = 32;
    mode.dmFields = DM_PELSWIDTH | DM_PELSHEIGHT | DM_BITSPERPEL;

    ChangeDisplaySettingsA(&mode, CDS_FULLSCREEN);
    ShowCursor(0);
}

unsafe fn create_device() -> HDC {
    let handle = CreateWindowExA(
        0,
        "edit\0".as_ptr(),
        0 as *const u8,
        WS_POPUP | WS_VISIBLE | WS_MAXIMIZE,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0 as *const c_void,
    );

    let device = GetDC(handle);
    let mut pfd: PIXELFORMATDESCRIPTOR = mem::zeroed();
    pfd.dwFlags = PFD_SUPPORT_OPENGL | PFD_DOUBLEBUFFER;
    let format = ChoosePixelFormat(device, &pfd);
    SetPixelFormat(device, format, &pfd);
    wglMakeCurrent(device, wglCreateContext(device));
    device
}

unsafe extern "C" fn wavesabre_device_factory(id: DeviceId) -> Device {
    match id {
        DeviceId::Slaughter => wavesabre_rs::device::slaughter(),
        _ => panic!(),
    }
}

#[no_mangle]
extern "C" fn mainCRTStartup() {
    unsafe {
        enter_fullscreen();
        let device = create_device();
        let program = gl::create_shader_program(gl::ShaderType::Fragment, glsl::SHADER_FRAG);
        gl::use_program(program);

        let length = wavesabre_rs::length(SONG_BLOB);
        let _player = wavesabre_rs::play(wavesabre_device_factory, &SONG_BLOB);

        while GetAsyncKeyState(VK_ESCAPE as i32) == 0 {
            let elapsed = time::elapsed();
            if elapsed > length {
                break;
            }

            gl::set_uniform(0, elapsed.as_secs_f32());
            glRects(-1, -1, 1, 1);
            SwapBuffers(device);
        }

        ExitProcess(0);
    }
}

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
