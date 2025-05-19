use crate::common::*;

use proc_macro2::TokenStream;
use quote::{
    format_ident,
    quote,
};

pub fn generate_lib(data: &serde_json::Value) -> TokenStream {
    let mut out = TokenStream::new();
    out.extend(generate_section(data, "enum", generate_enum));
    out.extend(generate_section(data, "bitmask", generate_bitmask));
    out.extend(generate_section(data, "structure", generate_struct));
    out.extend(generate_section(data, "callback function", generate_callback));
    out.extend(generate_section(data, "function", generate_function));
    out.extend(generate_section(data, "function pointer", generate_function_pointer));
    out.extend(generate_section(data, "object", generate_object));
    out
}

fn generate_enum(name: &Name, data: &serde_json::Value) -> TokenStream {
    let ident = format_ident!("WGPU{}", name.pascal_case());
    let items = data["values"]
        .as_array()
        .expect(E_JSON_ARRAY_EXPECTED)
        .iter()
        .map(|data| generate_enum_item(name, data));

    let last_word = name
        .words()
        .last()
        .expect("unexpected empty name");
    let emit_is = [
        "type",
        "reason",
        "status",
    ].contains(&last_word);

    let attr_is = if emit_is {
        quote!(#[cfg_attr(feature = "strum", derive(strum::EnumIs))])
    } else {
        quote!()
    };

    quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[cfg_attr(feature = "strum", derive(
            strum::Display,
            strum::EnumString,
            strum::EnumIter,
            strum::FromRepr,
            strum::IntoStaticStr))]
        #attr_is
        #[repr(i32)]
        pub enum #ident {
            #(#items)*
        }

        impl #ident {
            pub fn to_str(self) -> &'static str {
                self.into()
            }
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
    quote!(#ident = raw::#raw_ident,)
}

fn generate_bitmask(name: &Name, data: &serde_json::Value) -> TokenStream {
    let type_ident = format_ident!("WGPU{}", name.pascal_case());
    let items = data["values"]
        .as_array()
        .expect(E_JSON_ARRAY_EXPECTED)
        .iter()
        .map(|data| generate_bitmask_item(name, data));

    quote! {
        bitflags::bitflags! {
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            pub struct #type_ident: WGPUFlags {
                #(#items)*
            }
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
    quote!(const #ident = raw::#raw_ident;)
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
