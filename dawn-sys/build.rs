use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let cargo_manifest_dir = get_env_var_as_path_buf("CARGO_MANIFEST_DIR");
    let source_binary_dir = cargo_manifest_dir.join("bin");
    let output_binary_dir = get_output_binary_dir();
    let copy_src = source_binary_dir.join("webgpu_dawn.dll");
    let copy_dst = output_binary_dir.join("webgpu_dawn.dll");
    eprintln!("copy_src: {}", copy_src.display());
    eprintln!("copy_dst: {}", copy_dst.display());

    println!("cargo:rustc-link-search=native={}", output_binary_dir.display());
    println!("cargo:rustc-link-lib=dylib=webgpu_dawn");
    println!("cargo:rerun-if-changed={}", copy_src.display());

    fs::copy(&copy_src, &copy_dst)
        .expect("failed to copy webgpu_dawn.dll");
}

fn get_env_var_as_path_buf(key: &str) -> PathBuf {
    env::var(key)
        .expect(&format!("{key} not set"))
        .into()
}

fn get_output_binary_dir() -> PathBuf {
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR not set");
    let mut dir = PathBuf::from(out_dir);
    while let Some(parent) = dir.parent() {
        dir = parent.to_path_buf();
        if (dir.ends_with("debug") || dir.ends_with("release"))
            && dir.join(".fingerprint").is_dir()
            && dir.join("build").is_dir()
            && dir.join("deps").is_dir()
            && dir.join("examples").is_dir()
            && dir.join("incremental").is_dir() {
            return dir;
        }
    }
    panic!("failed to detect output binary directory, terminating ...");
}
