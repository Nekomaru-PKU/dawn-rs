use crate::{common::*, workspace_path};

use std::collections::BTreeSet;
use std::fs;

use proc_macro2::TokenStream;
use quote::{
    format_ident,
    quote,
};

pub fn generate_lib(data: &serde_json::Value) -> TokenStream {
    let mut out = quote!();
    out.extend(generate_section(data, "enum", generate_enum));
    out.extend(generate_section(data, "bitmask", generate_bitmask));
    out.extend(generate_section(data, "structure", generate_struct));
    out.extend(generate_section(data, "callback function", generate_callback));
    out.extend(generate_section(data, "function", generate_function));
    out.extend(generate_section(data, "function pointer", generate_function_pointer));
    out.extend(generate_section(data, "object", generate_object));
    assert!({
        get_all_functions()
            .difference(&get_generated_functions(data))
            .all(|name| name.ends_with("FreeMembers"))
    });
    get_all_functions()
        .difference(&get_generated_functions(data))
        .map(|name| format_ident!("{name}"))
        .for_each(|name| out.extend(quote!(pub use raw::#name;)));
    out
}

fn get_all_functions() -> BTreeSet<String> {
    fs::read_to_string(workspace_path!("dawn-sys/generated/bindings.rs"))
        .expect("failed to read bindings.rs")
        .lines()
        .filter_map(|line| line.strip_prefix("    pub fn wgpu"))
        .filter_map(|line| line.split_once('('))
        .map(|(name, _)| format!("wgpu{name}"))
        .collect()
}

fn get_generated_functions(data: &serde_json::Value) -> BTreeSet<String> {
    let data = data.as_object()
        .expect(E_JSON_OBJECT_EXPECTED);
    let mut list = BTreeSet::new();
    data.iter()
        .filter(|&(_, data)| data["category"].as_str() == Some("function"))
        .map(|(name, _)| name.clone())
        .for_each(|name| {
            list.insert(format!("wgpu{}", Name::new(name).pascal_case()));
        });
    data.iter()
        .filter(|&(_, data)| data["category"].as_str() == Some("object"))
        .flat_map(|(type_name, data)| {
            data["methods"]
                .as_array()
                .expect(E_JSON_ARRAY_EXPECTED)
                .iter()
                .map(|data| {
                    data["name"]
                    .as_str()
                    .expect(E_JSON_STRING_EXPECTED)
                })
                .chain(["add ref", "release"])
                .map(|method_name| (
                    type_name.clone(),
                    method_name.to_owned()))
        })
        .map(|(type_name, method_name)| {
            format!(
                "wgpu{}{}",
                Name::new(type_name).pascal_case(),
                Name::new(method_name).pascal_case())
        })
        .for_each(|name| {
            list.insert(name);
        });
    list
}

fn generate_enum(name: &Name, data: &serde_json::Value) -> TokenStream {
    let ident = format_ident!("WGPU{}", name.pascal_case());
    let items = data["values"]
        .as_array()
        .expect(E_JSON_ARRAY_EXPECTED)
        .iter()
        .map(|data| generate_enum_item(name, data));
    quote! {
        pub mod #ident {
            #![allow(non_snake_case)]
            #![allow(non_upper_case_globals)]

            use crate::raw;

            #(#items)*
        }
    }
}

fn generate_enum_item(type_name: &Name, data: &serde_json::Value) -> TokenStream {
    let name = Name::new({
        data["name"]
            .as_str()
            .expect(E_JSON_STRING_EXPECTED)
            .to_owned()
    });
    let raw_ident = format_ident!(
        "WGPU{}_WGPU{}_{}",
        type_name.pascal_case(),
        type_name.pascal_case(),
        name.pascal_case());
    let ident = format_ident!(
        "{}",
        name.handle_invalid_ident().pascal_case());
    quote!(pub const #ident: i32 = raw::#raw_ident;)
}

fn generate_bitmask(name: &Name, data: &serde_json::Value) -> TokenStream {
    let ident = format_ident!("WGPU{}", name.pascal_case());
    let items = data["values"]
        .as_array()
        .expect(E_JSON_ARRAY_EXPECTED)
        .iter()
        .map(|data| generate_bitmask_item(name, data));

    quote! {
        pub mod #ident {
            #![allow(non_snake_case)]

            use crate::raw;

            #(#items)*
        }
    }
}

fn generate_bitmask_item(type_name: &Name, data: &serde_json::Value) -> TokenStream {
    let name = Name::new({
        data["name"]
            .as_str()
            .expect(E_JSON_STRING_EXPECTED)
            .to_owned()
    });
    
    let ident = format_ident!("{}", name.screaming_snake_case());
    let raw_ident = format_ident!(
        "WGPU{}_{}",
        type_name.pascal_case(),
        name.pascal_case());
    quote!(pub const #ident: u64 = raw::#raw_ident;)
}

fn generate_struct(name: &Name, _: &serde_json::Value) -> TokenStream {
    let ident = format_ident!("WGPU{}", name.pascal_case());
    quote!(pub use raw::#ident;)
}

fn generate_callback(name: &Name, _: &serde_json::Value) -> TokenStream {
    let ident = format_ident!("WGPU{}", name.pascal_case());
    let ident_info = format_ident!("WGPU{}Info", name.pascal_case());
    quote! {
        pub use raw::#ident;
        pub use raw::#ident_info;
    }
}

fn generate_function(name: &Name, _: &serde_json::Value) -> TokenStream {
    let ident = format_ident!("wgpu{}", name.pascal_case());
    quote!(pub use raw::#ident;)
}
fn generate_function_pointer(name: &Name, _: &serde_json::Value) -> TokenStream {
    let ident = format_ident!("WGPU{}", name.pascal_case());
    quote!(pub use raw::#ident;)
}

fn generate_object(name: &Name, data: &serde_json::Value) -> TokenStream {
    let type_ident = format_ident!("WGPU{}", name.pascal_case());
    let method_ident_add_ref = format_ident!("wgpu{}AddRef", name.pascal_case());
    let method_ident_release = format_ident!("wgpu{}Release", name.pascal_case());
    let methods = data["methods"]
        .as_array()
        .expect(E_JSON_ARRAY_EXPECTED)
        .iter()
        .map(|data| generate_object_method(name, data));
    quote! {
        pub use raw::#type_ident;
        pub use raw::#method_ident_add_ref;
        pub use raw::#method_ident_release;
        #(#methods)*
    }
}

fn generate_object_method(type_name: &Name, data: &serde_json::Value) -> TokenStream {
    let name = Name::new({
        data["name"]
            .as_str()
            .expect(E_JSON_STRING_EXPECTED)
            .to_owned()
    });
    let ident = format_ident!(
        "wgpu{}{}",
        type_name.pascal_case(),
        name.pascal_case());
    quote!(pub use raw::#ident;)
}
