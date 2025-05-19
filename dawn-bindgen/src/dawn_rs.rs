#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]

use proc_macro2::TokenStream;
use quote::{
    format_ident,
    quote,
};

use crate::common::*;

pub fn generate_lib(data: &serde_json::Value) -> TokenStream {
    let mut out = quote!();
    out.extend(generate_section(data, "enum", generate_enum));
    out.extend(generate_section(data, "bitmask", generate_bitmask));
    out.extend(generate_section(data, "object", generate_object));
    out
}

fn generate_enum(name: &Name, data: &serde_json::Value) -> TokenStream {
    if  name.pascal_case().ends_with("ErrorType") ||
        name.pascal_case().ends_with("Reason") ||
        name.pascal_case().ends_with("Status") {
        return quote!();
    }

    let type_ident = format_ident!("{}", name.pascal_case());
    let type_raw_ident = format_ident!("WGPU{}", name.pascal_case());
    let items = data["values"]
        .as_array()
        .expect(E_JSON_ARRAY_EXPECTED)
        .iter()
        .map(|data| {
            let name = Name::new({
                data["name"]
                    .as_str()
                    .expect(E_JSON_STRING_EXPECTED)
                    .to_owned()
            });
            let ident = format_ident!(
                "{}",
                name.handle_invalid_ident().pascal_case());
            quote!(#ident = sys::#type_raw_ident::#ident)
        });
    let iters = data["values"]
        .as_array()
        .expect(E_JSON_ARRAY_EXPECTED)
        .iter()
        .map(|data| {
            let name = Name::new({
                data["name"]
                    .as_str()
                    .expect(E_JSON_STRING_EXPECTED)
                    .to_owned()
            });
            let ident = format_ident!(
                "{}",
                name.handle_invalid_ident().pascal_case());
            quote!(Self::#ident)
        });
    quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[derive(
            strum::Display,
            strum::EnumString,
            strum::FromRepr,
            strum::IntoStaticStr)]
        #[repr(i32)]
        pub enum #type_ident {
            #(#items),*
        }

        impl #type_ident {
            pub fn to_str(self) -> &'static str {
                self.into()
            }

            pub fn iter() -> impl Iterator<Item = Self> {
                [ #(#iters),*].into_iter()
            }
        }
    }
}

fn generate_bitmask(name: &Name, data: &serde_json::Value) -> TokenStream {
    let type_ident = format_ident!("WGPU{}", name.pascal_case());
    let type_raw_ident = format_ident!("WGPU{}", name.pascal_case());
    let items = data["values"]
        .as_array()
        .expect(E_JSON_ARRAY_EXPECTED)
        .iter()
        .map(|data| {
            let name = Name::new({
                data["name"]
                    .as_str()
                    .expect(E_JSON_STRING_EXPECTED)
                    .to_owned()
            });
            let ident = format_ident!(
                "{}",
                name.handle_invalid_ident().screaming_snake_case());
            quote!(const #ident = sys::#type_raw_ident::#ident;)
        });
    quote! {
        bitflags::bitflags! {
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            pub struct #type_ident: sys::WGPUFlags {
                #(#items)*
            }
        }
    }
}

fn generate_object(name: &Name, data: &serde_json::Value) -> TokenStream {
    let ident = format_ident!("{}", name.pascal_case());
    let ident_sys = format_ident!("WGPU{}", name.pascal_case());
    let method_ident_add_ref = format_ident!("wgpu{}AddRef", name.pascal_case());
    let method_ident_release = format_ident!("wgpu{}Release", name.pascal_case());

    quote!{
        pub struct #ident(sys::#ident_sys);
        impl Clone for #ident {
            fn clone(&self) -> Self {
                unsafe {
                    sys::#method_ident_add_ref(self.0);
                }
                Self(self.0)
            }
        }

        impl Drop for #ident {
            fn drop(&mut self) {
                unsafe {
                    sys::#method_ident_release(self.0);
                }
            }
        }
    }
}
