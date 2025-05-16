use proc_macro2::TokenStream;
use yaml_rust2::Yaml;

use quote::{
    format_ident,
    quote,
};

use crate::error_yaml_invalid_field;
use crate::common::{
    generate_section_from_yaml,
    get_name_from_yaml,
    handle_invalid_ident,
    snake_case_to_pascal_case,
    snake_case_to_screaming_snake_case,
};

pub fn generate_from_yaml(yaml: &Yaml) -> TokenStream {
    let mut out = quote! {
        pub use raw::WGPUFlags;
        pub use raw::WGPUBool;
    };

    out.extend(generate_section_from_yaml(yaml, "enums", generate_enum));
    out.extend(generate_section_from_yaml(yaml, "bitflags", generate_bitflag));
    out.extend(generate_section_from_yaml(yaml, "structs", generate_struct));
    out.extend(generate_section_from_yaml(yaml, "callbacks", generate_callback));
    out.extend(generate_section_from_yaml(yaml, "functions", generate_function));
    out.extend(generate_section_from_yaml(yaml, "objects", generate_object));
    out
}

fn generate_enum(yaml: &Yaml) -> TokenStream {
    let type_name = snake_case_to_pascal_case(get_name_from_yaml(yaml));
    let type_ident = format_ident!("WGPU{type_name}");

    let items = yaml["entries"]
        .as_vec()
        .expect(error_yaml_invalid_field!("entries"))
        .iter()
        .filter(|&item| !item.is_null())
        .map(|yaml| generate_enum_item(&type_name, yaml));

    quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[cfg_attr(feature = "strum", derive(
            strum::Display,
            strum::EnumString,
            strum::FromRepr,
            strum::IntoStaticStr))]
        #[repr(u32)]
        pub enum #type_ident {
            #(#items)*
        }

        impl #type_ident {
            pub fn to_str(self) -> &'static str {
                self.into()
            }
        }
    }
}

fn generate_enum_item(type_name: &str, yaml: &Yaml) -> TokenStream {
    let snake_case_name = get_name_from_yaml(yaml);
    let name = snake_case_to_pascal_case(
        &handle_invalid_ident(snake_case_name));
    let name_raw = snake_case_to_pascal_case(snake_case_name);
    let ident = format_ident!("{name}");
    let ident_raw = format_ident!(
        "WGPU{}_WGPU{}_{}",
        type_name,
        type_name,
        name_raw);
    quote!(#ident = raw::#ident_raw as _,)
}

fn generate_bitflag(yaml: &Yaml) -> TokenStream {
    let type_name = snake_case_to_pascal_case(get_name_from_yaml(yaml));
    let type_ident = format_ident!("WGPU{type_name}");

    let items = yaml["entries"]
        .as_vec()
        .expect(error_yaml_invalid_field!("entries"))
        .iter()
        .filter(|&item| !item.is_null())
        .map(|yaml| generate_bitflag_item(&type_name, yaml));

    quote! {
        bitflags::bitflags! {
            #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
            #[repr(transparent)]
            pub struct #type_ident: WGPUFlags {
                #(#items)*
            }
        }
    }
}

fn generate_bitflag_item(type_name: &str, yaml: &Yaml) -> TokenStream {
    let snake_case_name = get_name_from_yaml(yaml);
    let name = snake_case_to_screaming_snake_case(snake_case_name);
    let name_raw = snake_case_to_pascal_case(snake_case_name);
    let ident = format_ident!("{name}");
    let ident_raw = format_ident!("WGPU{type_name}_{name_raw}");
    quote!(const #ident = raw::#ident_raw as _;)
}

fn generate_struct(yaml: &Yaml) -> TokenStream {
    let name = snake_case_to_pascal_case(get_name_from_yaml(yaml));
    let ident = format_ident!("WGPU{name}");
    quote!(pub use raw::#ident;)
}

fn generate_callback(yaml: &Yaml) -> TokenStream {
    let name = snake_case_to_pascal_case(get_name_from_yaml(yaml));
    let ident = format_ident!("WGPU{name}Callback");
    quote!(pub use raw::#ident;)
}

fn generate_function(yaml: &Yaml) -> TokenStream {
    let name = snake_case_to_pascal_case(get_name_from_yaml(yaml));
    let ident = format_ident!("wgpu{name}");
    quote!(pub use raw::#ident;)
}

fn generate_object(yaml: &Yaml) -> TokenStream {
    let type_name = snake_case_to_pascal_case(get_name_from_yaml(yaml));
    let type_ident = format_ident!("WGPU{type_name}");

    let mut out = quote!(pub use raw::#type_ident;);
    yaml["methods"]
        .as_vec()
        .expect(error_yaml_invalid_field!("methods"))
        .iter()
        .filter(|&item| !item.is_null())
        .map(|yaml| generate_object_method(&type_name, yaml))
        .for_each(|item| out.extend(item));
    out
}

fn generate_object_method(type_name: &str, yaml: &Yaml) -> TokenStream {
    let name = snake_case_to_pascal_case(get_name_from_yaml(yaml));
    let ident = format_ident!("wgpu{type_name}{name}");
    quote!(pub use raw::#ident;)
}
