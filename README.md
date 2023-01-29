# 64k-starter

This is a template repository that can be used as a starting point when making (somewhat) small demoscene productions in
Rust.

### Out of the box it will:

- Minify shader sources as a build step using [Shader_Minifier](https://github.com/laurentlb/Shader_Minifier)
- Create a fullscreen 1920x1080 window
- Load a fragment shader (the default example from [Shadertoy](https://www.shadertoy.com/))
- Load and play a [WaveSabre](https://github.com/logicomacorp/WaveSabre) song using [wavesabre-rs](https://github.com/Kwarf/wavesabre-rs)
- Update the shader uniform (iTime) each frame
- Render until the song ends

### Size

A release build (`cargo build --release`) of this project as is will result in a binary size of 27 136 bytes.

Packing it with [UPX](https://github.com/upx/upx) can then reduce it down to 15 360 bytes.

#### Without music

If you have a demo without music, or if you bring your own software synth, the size without music may be of interest.

3 584 bytes as is, but with some more linker tweaks it can be brought down further to 2 192 bytes, which UPX cannot
improve.

## Thanks to

- [in4k/isystem1k4k](https://github.com/in4k/isystem1k4k) for showing the bare minimum Win32 calls required to set everything up.
- [mcountryman/min-sized-rust-windows](https://github.com/mcountryman/min-sized-rust-windows) for the linker flags I use in `build.rs` to reduce the size of the binary.
