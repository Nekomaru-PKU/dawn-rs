#[macro_export] macro_rules! workspace_path {
    ($path:literal) => {
        concat!(env!("CARGO_MANIFEST_DIR"), "/../", $path)
    };
}

#[macro_export] macro_rules! error_yaml_invalid_field {
    ($field:literal) => {
        concat!("failed to parse webgpu.yml: invalid `", $field, "` field")
    };
}

use proc_macro2::TokenStream;
use yaml_rust2::Yaml;

pub fn handle_invalid_ident(ident: &str) -> String {
    if let Some(prefix) = ident.get(..2) && ["1D", "2D", "3D"].contains(&prefix) {
        format!("D{}{}", &ident[..1], &ident[2..])
    } else {
        ident.to_owned()
    }
}

pub fn get_name_from_yaml(yaml: &Yaml) -> &str {
    yaml["name"]
        .as_str()
        .expect(error_yaml_invalid_field!("name"))
}

pub fn generate_section_from_yaml<F>(
    yaml: &Yaml,
    name: &str,
    generate: F)
 -> TokenStream
where
    F: Fn(&Yaml) -> TokenStream, {
    let section = yaml[name]
        .as_vec()
        .unwrap_or_else(|| panic!("failed to parse webgpu.yml: invalid `{name}` field"));
    let mut out = TokenStream::new();
    for item in section {
        out.extend(generate(item));
    }
    out
}

pub fn snake_case_to_pascal_case(s: &str) -> String {
    if let Some(stripped) = s.strip_prefix("_10_10_2") {
        return format!("{stripped}10_10_2");
    }

    let mut out = String::new();
    let mut start = true;
    for c in s.chars() {
        match (c, start) {
            ('_', true) => out.push('_'),
            ('_', false) => start = true,
            (_, true) => {
                out.push(c.to_ascii_uppercase());
                start = false;
            },
            (_, false) => out.push(c),
        }
    }
    out
}

pub fn snake_case_to_screaming_snake_case(s: &str) -> String {
    s.chars().map(|c| c.to_ascii_uppercase()).collect()
}

