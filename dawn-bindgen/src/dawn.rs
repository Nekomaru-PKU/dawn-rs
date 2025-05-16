#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]

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

    out
}
