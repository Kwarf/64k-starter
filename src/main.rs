#![no_std]
#![no_main]
#![windows_subsystem = "console"]

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
            glRects, wglCreateContext, wglGetProcAddress, wglMakeCurrent, ChoosePixelFormat,
            SetPixelFormat, SwapBuffers, PFD_DOUBLEBUFFER, PFD_SUPPORT_OPENGL,
            PIXELFORMATDESCRIPTOR,
        },
    },
    System::{SystemInformation::GetTickCount, Threading::ExitProcess},
    UI::{
        Input::KeyboardAndMouse::{GetAsyncKeyState, VK_ESCAPE},
        WindowsAndMessaging::{CreateWindowExA, ShowCursor, WS_MAXIMIZE, WS_POPUP, WS_VISIBLE},
    },
};

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
        "edit".as_ptr(),
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

unsafe fn set_shader(source: &'static str) {
    let create_program: unsafe extern "C" fn(i32, u32, &*const u8) -> i32 = core::mem::transmute(
        wglGetProcAddress(b"glCreateShaderProgramv\0".as_ptr() as *const u8).unwrap(),
    );
    let use_program: unsafe extern "C" fn(i32) -> c_void =
        core::mem::transmute(wglGetProcAddress(b"glUseProgram\0".as_ptr() as *const u8).unwrap());

    let program = create_program(0x8B30, 1, &source.as_ptr());
    use_program(program);
}

unsafe fn set_uniform(location: i32, value: f32) {
    let set: unsafe extern "C" fn(i32, f32) =
        core::mem::transmute(wglGetProcAddress(b"glUniform1f\0".as_ptr() as *const u8).unwrap());
    set(location, value);
}

unsafe fn elapsed_seconds() -> f32 {
    static mut START: u32 = 0;
    if START == 0 {
        START = GetTickCount();
    }
    (GetTickCount() - START) as f32 / 1000f32
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
        set_shader("uniform float iTime;\nvoid main(){gl_FragColor=vec4(.5+.5*cos(iTime+(gl_FragCoord.xy/vec2(1920,1080)).xyx+vec3(0,2,4)),1.0);}\0");

        let length = wavesabre_rs::length(SONG_BLOB);
        let _player = wavesabre_rs::play(wavesabre_device_factory, &SONG_BLOB);

        while GetAsyncKeyState(VK_ESCAPE as i32) == 0 {
            let elapsed = elapsed_seconds();
            if elapsed > length.as_secs_f32() {
                break;
            }

            set_uniform(0, elapsed);
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
