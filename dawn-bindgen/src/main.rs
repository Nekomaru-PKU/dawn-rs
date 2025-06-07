macro_rules! workspace_path {
    ($path:literal) => (concat!(env!("CARGO_MANIFEST_DIR"), "/../", $path));
}

use proc_macro2::TokenStream;
use serde_json::Value as JsonValue;

use quote::quote;
use quote::format_ident;

const DAWN_JSON: &str = include_str!(
    workspace_path!("dawn-bindgen/include/dawn.json"));
const DAWN_ITEM_CATEGORIES: &[&str] = &[
    "bitmask",
    "callback function",
    "callback info",
    "constant",
    "enum",
    "function pointer",
    "function",
    "object",
    "structure",
    "typedef",
];

fn main() {
    let metadata = serde_json::from_str(DAWN_JSON).unwrap();
    bindings::save_to_file(
        &metadata,
        workspace_path!("dawn-bindgen/include/dawn.h"),
        workspace_path!("dawn-sys/generated/bindings.rs"));
    save_token_stream_to_file(
        dawn_sys::generate_bitmasks(&metadata),
        workspace_path!("dawn-sys/generated/bitmasks.rs"));
    save_token_stream_to_file(
        dawn_sys::generate_lib(&metadata),
        workspace_path!("dawn-sys/generated/lib.rs"));
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

mod name {
    pub fn to_pascal_case(name: &str) -> String {
        name.split_whitespace()
            .map(|word| {
                let first_char = word
                    .chars()
                    .next()
                    .expect("unexpected empty word")
                    .to_ascii_uppercase();
                Some(first_char)
                    .into_iter()
                    .chain(word.chars().skip(1))
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("")
    }

    pub fn to_screaming_snake_case(name: &str) -> String {
        name.split_whitespace()
            .map(|word| word.to_ascii_uppercase())
            .collect::<Vec<_>>()
            .join("_")
    }
}

mod metadata {
    use crate::*;

    pub fn get_items_by_category<'a>(root: &'a JsonValue, category: &str)
     -> Vec<(&'a str, &'a JsonValue)> {
        let category = JsonValue::String(category.to_owned());
        root.as_object()
            .expect("failed to parse dawn.json: root is not object")
            .iter()
            .map(|(name, data)| (name.as_ref(), data))
            .filter(|&(_, item)| item["category"] == category)
            .collect()
    }

    pub fn get_methods(item: &JsonValue) -> Vec<(&str, &JsonValue)> {
        item.as_object()
            .expect("failed to parse dawn.json: item is not object")
            .get("methods")
            .expect("failed to parse dawn.json: object has no methods")
            .as_array()
            .expect("failed to parse dawn.json: methods is not array")
            .iter()
            .map(|method| {
                let name = method["name"]
                    .as_str()
                    .expect("failed to parse dawn.json: method name is not string");
                (name, method)
            })
            .collect()
    }
}

mod bindings {
    use crate::*;

    pub fn save_to_file(
        root: &JsonValue,
        header_path: &str,
        output_path: &str) {
        let builder = bindgen::builder()
            .header(header_path)
            .use_core()
            .derive_default(true)
            .derive_eq(true)
            .derive_ord(true)
            .derive_hash(true)
            .default_enum_style(bindgen::EnumVariation::NewType {
                is_bitfield: false,
                is_global: true,
            });
        let builder = get_bitmask_and_bitmast_constant_names(root)
            .iter()
            .fold(builder, bindgen::Builder::blocklist_item);
        builder
            .generate()
            .expect("failed to generate bindings for dawn.h")
            .write_to_file(output_path)
            .unwrap_or_else(|err| panic!("failed to write file {output_path}: {err}"));
    }

    fn get_bitmask_and_bitmast_constant_names(root: &JsonValue) -> Vec<String> {
        metadata::get_items_by_category(root, "bitmask")
            .iter()
            .flat_map(|(type_name, item)| {
                Some(format!("WGPU{}", name::to_pascal_case(type_name)))
                    .into_iter()
                    .chain({
                        item["values"]
                            .as_array()
                            .expect("failed to parse dawn.json: values is not array")
                            .iter()
                            .map(|data| {
                                data["name"]
                                    .as_str()
                                    .expect("failed to parse dawn.json: name is not string")
                            })
                            .map(|name| {
                                format!(
                                    "WGPU{}_{}",
                                    name::to_pascal_case(type_name),
                                    name::to_pascal_case(name))
                        })
                })
            })
            .collect()
    }
}

mod dawn_sys {
    use crate::*;

    const EXTRA_EXPORT_TYPES: &[&str] = &[
        "Bool",
        "ChainedStruct",
        "Flags",
    ];

    const EXTRA_EXPORT_FUNCTIONS: &[&str] = &[
        "AdapterInfoFreeMembers",
        "AdapterPropertiesMemoryHeapsFreeMembers",
        "AdapterPropertiesSubgroupMatrixConfigsFreeMembers",
        "DawnDrmFormatCapabilitiesFreeMembers",
        "SharedBufferMemoryEndAccessStateFreeMembers",
        "SharedTextureMemoryEndAccessStateFreeMembers",
        "SupportedFeaturesFreeMembers",
        "SupportedWGSLLanguageFeaturesFreeMembers",
        "SurfaceCapabilitiesFreeMembers",
    ];

    const EXCLUDE_CATEGORIES: &[&str] = &[
        "constant",
    ];

    const EXCLUDE_EXPORTS: &[&str] = &[
        "WGPUINTERNAL_HAVE_EMDAWNWEBGPU_HEADER",
    ];

    pub fn generate_bitmasks(root: &JsonValue) -> TokenStream {
        let mut out = TokenStream::new();
        for (name, data) in metadata::get_items_by_category(root, "bitmask") {
            out.extend(generate_bitmask(name, data));
        }
        out
    }

    fn generate_bitmask(type_name: &str, data: &JsonValue) -> TokenStream {
        let type_name = name::to_pascal_case(type_name);
        let type_ident = format_ident!("WGPU{type_name}");
        let items = data["values"]
            .as_array()
            .expect("failed to parse dawn.json: values is not array")
            .iter()
            .map(|data| {
                let name = data["name"]
                    .as_str()
                    .expect("failed to parse dawn.json: name is not string")
                    .to_owned();
                let ident = format_ident!("{}", name::to_screaming_snake_case(&name));
                let value = data["value"]
                    .as_u64()
                    .expect("failed to parse dawn.json: value is not u64");
                quote!(const #ident = #value;)
            });
        quote! {
            ::bitflags::bitflags! {
                #[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
                #[repr(transparent)]
                pub struct #type_ident: WGPUFlags {
                    #(#items)*
                }
            }
        }

    }

    pub fn generate_lib(data: &JsonValue) -> TokenStream {
        let mut out = TokenStream::new();
        get_exports(data)
            .into_iter()
            .for_each(|ident| out.extend(quote!(pub use raw::#ident;)));
        metadata::get_items_by_category(data, "enum")
            .into_iter()
            .for_each(|(name, data)| out.extend(generate_enum_impl(name, data)));
        out
    }

    fn get_exports(data: &JsonValue) -> Vec<syn::Ident> {
        let mut exports = Vec::new();

        for &category in DAWN_ITEM_CATEGORIES {
            if EXCLUDE_CATEGORIES.contains(&category) {
                continue;
            }

            for (name, data) in metadata::get_items_by_category(data, category) {
                for ident in get_exports_for_item(name, category) {
                    exports.push(ident);
                }

                if category == "object" {
                    for (method_name, _) in metadata::get_methods(data) {
                        exports.extend(get_exports_for_method(name, method_name));
                    }
                }
            }
        }

        for &name in EXTRA_EXPORT_TYPES {
            exports.push(format_ident!("WGPU{}", name));
        }

        for &name in EXTRA_EXPORT_FUNCTIONS {
            exports.push(format_ident!("wgpu{}", name));
            exports.push(format_ident!("WGPUProc{}", name));
        }

        for &name in EXCLUDE_EXPORTS {
            let pos = exports
                .iter()
                .position(|ident| ident == &name)
                .expect("missing export in EXCLUDE_EXPORTS");
            exports.remove(pos);
        }

        exports.sort_unstable_by_key(|ident| ident.to_string().to_ascii_lowercase());
        exports
    }

    fn get_exports_for_item(name: &str, category: &str) -> Vec<syn::Ident> {
        let name_pascal = name::to_pascal_case(name);
        match category {
            "constant" => vec![
                format_ident!("WGPU_{}", name::to_screaming_snake_case(name)),
            ],
            "function" => vec![
                format_ident!("wgpu{}", name_pascal),
                format_ident!("WGPUProc{}", name_pascal),
            ],
            "object" => vec![
                format_ident!("WGPU{}", name_pascal),
                format_ident!("WGPU{}Impl", name_pascal),
                format_ident!("WGPUProc{}AddRef", name_pascal),
                format_ident!("WGPUProc{}Release", name_pascal),
                format_ident!("wgpu{}AddRef", name_pascal),
                format_ident!("wgpu{}Release", name_pascal),
            ],
            _ => vec![
                format_ident!("WGPU{}", name_pascal),
            ],
        }
    }

    fn get_exports_for_method(object_name: &str, method_name: &str) -> Vec<syn::Ident> {
        let object_name = name::to_pascal_case(object_name);
        let method_name = name::to_pascal_case(method_name);
        vec![
            format_ident!("wgpu{}{method_name}", object_name),
            format_ident!("WGPUProc{}{method_name}", object_name),
        ]
    }

    fn generate_enum_impl(type_name: &str, data: &JsonValue) -> TokenStream {
        let type_name = name::to_pascal_case(type_name);
        let type_ident = format_ident!("WGPU{type_name}");
        let items = data["values"]
            .as_array()
            .expect("failed to parse dawn.json: values is not array")
            .iter()
            .map(|data| {
                let name = data["name"]
                    .as_str()
                    .expect("failed to parse dawn.json: name is not string")
                    .to_owned();
                let raw_name = name::to_pascal_case(&name);
                let raw_ident = format_ident!("WGPU{type_name}_WGPU{type_name}_{raw_name}");
                let export_name = match raw_name.get(0..2) {
                    Some("1D") => format!("D1{}", &raw_name[2..]),
                    Some("2D") => format!("D2{}", &raw_name[2..]),
                    Some("3D") => format!("D3{}", &raw_name[2..]),
                    _ => raw_name,
                };
                let export_ident = format_ident!("{export_name}");
                quote!(pub const #export_ident: Self = raw::#raw_ident;)
            });
        quote!(impl #type_ident { #(#items)* })
    }
}
