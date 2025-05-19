mod common;
mod dawn_sys;
mod dawn_rs;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let args = args.iter().map(String::as_str).collect::<Vec<_>>();

    let mut generate_bindings = true;
    match args.as_slice() {
        [_] => (),
        [_, "--skip-bindings"] =>
            generate_bindings = false,
        _ => panic!("invalid arguments: {args:?}"),
    }

    if generate_bindings {
        save_bindings_to_file(
            workspace_path!("dawn-sys/generated/bindings.rs"));
    }

    let json_str = include_str!(
        workspace_path!("dawn-bindgen/include/dawn.json"));
    let json = serde_json::from_str(json_str)
        .expect("failed to parse dawn.json");
    save_token_stream_to_file(
        dawn_sys::generate_lib(&json),
        workspace_path!("dawn-sys/generated/lib.rs"));
    save_token_stream_to_file(
        dawn_rs::generate_lib(&json),
        workspace_path!("dawn-rs/generated/lib.rs"));
}

fn save_bindings_to_file(path: &str) {
    bindgen::builder()
        .header(workspace_path!("dawn-bindgen/include/dawn.h"))
        .use_core()
        .generate()
        .expect("failed to generate bindings for dawn.h")
        .write_to_file(path)
        .unwrap_or_else(|err| panic!("failed to write file {path}: {err}"));
}

fn save_token_stream_to_file(
    token_stream: proc_macro2::TokenStream,
    path: &str) {
    let ast = syn::parse2(token_stream)
        .expect("failed to parse generated token stream");
    let out = prettyplease::unparse(&ast);
    std::fs::write(path, out)
        .unwrap_or_else(|err| panic!("failed to write file {path:?}: {err}"));
}
