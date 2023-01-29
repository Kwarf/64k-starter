use std::{
    env,
    ffi::OsStr,
    fs::File,
    path::{Path, PathBuf},
    process::Command,
};

use walkdir::WalkDir;

fn shader_minifier_path() -> PathBuf {
    Path::new(&env::var_os("CARGO_MANIFEST_DIR").unwrap())
        .join("target")
        .join("shader_minifier.exe")
        .to_path_buf()
}

fn ensure_shader_minifier_exists() {
    let path = shader_minifier_path();
    if !path.exists() {
        let mut response = reqwest::blocking::get("https://github.com/laurentlb/Shader_Minifier/releases/download/1.3.3/shader_minifier.exe")
            .unwrap();

        let mut file = File::create(path).unwrap();
        response.copy_to(&mut file).unwrap();
    }
}

fn minify_shaders() {
    let sources: Vec<PathBuf> = WalkDir::new(env::var_os("CARGO_MANIFEST_DIR").unwrap())
        .into_iter()
        .filter_map(|x| x.ok())
        .filter(|x| {
            let extensions: &[&str] = &["glsl", "frag", "vert"];
            if let Some(extension) = x.path().extension().and_then(OsStr::to_str) {
                return extensions.iter().any(|x| x.eq_ignore_ascii_case(extension));
            }

            false
        })
        .map(|x| x.into_path())
        .collect();

    Command::new(shader_minifier_path())
        .args([
            "-o",
            Path::new(&env::var_os("CARGO_MANIFEST_DIR").unwrap())
                .join("src")
                .join("glsl.rs")
                .to_str()
                .unwrap(),
        ])
        .args(["--format", "rust"])
        .args(sources.iter().map(|x| x.as_os_str()))
        .output()
        .unwrap();

    for x in &sources {
        println!("cargo:rerun-if-changed={}", x.display());
    }
}

fn main() {
    ensure_shader_minifier_exists();
    minify_shaders();

    println!("cargo:rustc-link-arg-bins=/DEBUG:NONE");
    println!("cargo:rustc-link-arg-bins=/EMITPOGOPHASEINFO");
    println!("cargo:rustc-link-arg-bins=/MERGE:.pdata=.text");
    println!("cargo:rustc-link-arg-bins=/MERGE:.rdata=.text");
    println!("cargo:rustc-link-arg-bins=/NODEFAULTLIB");
}
