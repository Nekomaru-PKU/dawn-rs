mod common;
mod dawn;
mod dawn_sys;

use proc_macro2::TokenStream;

fn main() {
    #[cfg(feature = "generate-bindings")]
    generate_bindings_to_file(
        workspace_path!("dawn-sys/generated/bindings.rs"));

    let yaml_str = include_str!(
        workspace_path!("dawn-bindgen/include/webgpu.yml"));
    let yaml = yaml_rust2::YamlLoader::load_from_str(yaml_str)
        .expect("failed to parse webgpu.yml")
        .into_iter()
        .next()
        .expect("failed to parse webgpu.yml: no document found");
    save_token_stream_to_file(
        dawn_sys::generate_from_yaml(&yaml),
        workspace_path!("dawn-sys/generated/lib.rs"));
}

#[cfg(feature = "generate-bindings")]
fn generate_bindings_to_file(path: &str) {
    std::fs::write(
        workspace_path!("dawn-bindgen/include/webgpu-no-docs.h"),
        include_str!(workspace_path!("dawn-bindgen/include/webgpu.h"))
            .lines()
            .filter(|line| !{
                let line = line.trim_start();
                line == "*" ||
                line.starts_with("* ") ||
                line.starts_with("/*") ||
                line.ends_with("*/")
            })
            .collect::<Vec<_>>()
            .join("\n"))
        .unwrap_or_else(|err| panic!("failed to write file {path}: {err}"));
    bindgen::builder()
        .header(workspace_path!("dawn-bindgen/include/webgpu-no-docs.h"))
        .use_core()
        .generate()
        .expect("failed to generate bindings for webgpu.h")
        .write_to_file(path)
        .unwrap_or_else(|err| panic!("failed to write file {path}: {err}"));
}

fn save_token_stream_to_file(
    token_stream: TokenStream,
    path: &str) {
    let ast = syn::parse2(token_stream)
        .expect("failed to parse generated token stream");
    let out = prettyplease::unparse(&ast);
    std::fs::write(path, out)
        .unwrap_or_else(|err| panic!("failed to write file {path:?}: {err}"));
}
