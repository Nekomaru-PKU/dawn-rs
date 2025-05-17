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
    let mut out = TokenStream::new();
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

    let items_vec = yaml["entries"]
        .as_vec()
        .expect(error_yaml_invalid_field!("entries"));
    let emit_is =
        type_name.ends_with("_type") ||
        type_name.ends_with("_reason") ||
        type_name.ends_with("_status");
    let emit_doc = items_vec
        .iter()
        .all(|item| {
            item["doc"]
                .as_str()
                .map(str::trim)
                .map(|doc| !doc.is_empty() && doc != "TODO")
                .unwrap_or(false)
        });
    let attr_is = if emit_is {
        quote!(#[cfg_attr(feature = "strum", derive(strum::EnumIs))])
    } else {
        quote!()
    };
    let items = items_vec
        .iter()
        .filter(|&item| !item.is_null())
        .map(|yaml| generate_enum_item(&type_name, yaml, emit_doc));

    quote! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        #[cfg_attr(feature = "strum", derive(
            strum::Display,
            strum::EnumString,
            strum::FromRepr,
            strum::IntoStaticStr))]
        #attr_is
        #[repr(i32)]
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

fn generate_enum_item(type_name: &str, yaml: &Yaml, emit_doc: bool) -> TokenStream {
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

    if emit_doc {
        let doc = yaml["doc"]
            .as_str()
            .expect(error_yaml_invalid_field!("doc"))
            .replace('"', "")
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join("\n\n");
        let doc = if doc.contains(
            "Indicates no value is passed for this argument.") {
            "Indicates no value is passed for this argument.".into()
        } else if doc.contains('\n') {
            format!("\n{}", doc)
        } else {
            doc
        };
        let doc = format!(" {doc}");
        quote! {
            #[doc = #doc]
            #ident = raw::#ident_raw,
        }
    } else {
        quote!(#ident = raw::#ident_raw,)
    }
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
    quote!(const #ident = raw::#ident_raw;)
}

fn generate_struct(yaml: &Yaml) -> TokenStream {
    let name = snake_case_to_pascal_case(get_name_from_yaml(yaml));
    let ident = format_ident!("WGPU{name}");
    quote!(pub use raw::#ident;)
}

fn generate_callback(yaml: &Yaml) -> TokenStream {
    let name = snake_case_to_pascal_case(get_name_from_yaml(yaml));
    let ident = format_ident!("WGPU{name}Callback");
    let ident_info = format_ident!("WGPU{name}CallbackInfo");
    quote!{
        pub use raw::#ident;
        pub use raw::#ident_info;
    }
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
