#![cfg(not(doc))]

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let args = args.iter().map(String::as_str).collect::<Vec<_>>();

    let mut skip_bindings = false;
    match args.as_slice() {
        [_] => (),
        [_, "--skip-bindings"] =>
            skip_bindings = true,
        _ => panic!("invalid arguments: {args:?}"),
    }

    if !skip_bindings {
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
    // save_token_stream_to_file(
    //     dawn_rs::generate_lib(&json),
    //     workspace_path!("dawn-rs/generated/lib.rs"));
}

fn save_bindings_to_file(path: &str) {
    let header_main = include_str!(workspace_path!("dawn-bindgen/include/dawn.h"))
        .lines()
        .filter(|line| {
            !line.starts_with("#include") &&
            !line.starts_with("#pragma")
        });
    let header_stdint = "
        typedef signed char        int8_t;
        typedef short              int16_t;
        typedef int                int32_t;
        typedef long long          int64_t;
        typedef unsigned char      uint8_t;
        typedef unsigned short     uint16_t;
        typedef unsigned int       uint32_t;
        typedef unsigned long long uint64_t;

        typedef signed char        int_least8_t;
        typedef short              int_least16_t;
        typedef int                int_least32_t;
        typedef long long          int_least64_t;
        typedef unsigned char      uint_least8_t;
        typedef unsigned short     uint_least16_t;
        typedef unsigned int       uint_least32_t;
        typedef unsigned long long uint_least64_t;

        typedef signed char        int_fast8_t;
        typedef int                int_fast16_t;
        typedef int                int_fast32_t;
        typedef long long          int_fast64_t;
        typedef unsigned char      uint_fast8_t;
        typedef unsigned int       uint_fast16_t;
        typedef unsigned int       uint_fast32_t;
        typedef unsigned long long uint_fast64_t;

        typedef long long          intmax_t;
        typedef unsigned long long uintmax_t;

        #define INT8_MIN         (-127i8 - 1)
        #define INT16_MIN        (-32767i16 - 1)
        #define INT32_MIN        (-2147483647i32 - 1)
        #define INT64_MIN        (-9223372036854775807i64 - 1)
        #define INT8_MAX         127i8
        #define INT16_MAX        32767i16
        #define INT32_MAX        2147483647i32
        #define INT64_MAX        9223372036854775807i64
        #define UINT8_MAX        0xffui8
        #define UINT16_MAX       0xffffui16
        #define UINT32_MAX       0xffffffffui32
        #define UINT64_MAX       0xffffffffffffffffui64

        #define INT_LEAST8_MIN   INT8_MIN
        #define INT_LEAST16_MIN  INT16_MIN
        #define INT_LEAST32_MIN  INT32_MIN
        #define INT_LEAST64_MIN  INT64_MIN
        #define INT_LEAST8_MAX   INT8_MAX
        #define INT_LEAST16_MAX  INT16_MAX
        #define INT_LEAST32_MAX  INT32_MAX
        #define INT_LEAST64_MAX  INT64_MAX
        #define UINT_LEAST8_MAX  UINT8_MAX
        #define UINT_LEAST16_MAX UINT16_MAX
        #define UINT_LEAST32_MAX UINT32_MAX
        #define UINT_LEAST64_MAX UINT64_MAX

        #define INT_FAST8_MIN    INT8_MIN
        #define INT_FAST16_MIN   INT32_MIN
        #define INT_FAST32_MIN   INT32_MIN
        #define INT_FAST64_MIN   INT64_MIN
        #define INT_FAST8_MAX    INT8_MAX
        #define INT_FAST16_MAX   INT32_MAX
        #define INT_FAST32_MAX   INT32_MAX
        #define INT_FAST64_MAX   INT64_MAX
        #define UINT_FAST8_MAX   UINT8_MAX
        #define UINT_FAST16_MAX  UINT32_MAX
        #define UINT_FAST32_MAX  UINT32_MAX
        #define UINT_FAST64_MAX  UINT64_MAX

        #ifdef _WIN64
            #define INTPTR_MIN   INT64_MIN
            #define INTPTR_MAX   INT64_MAX
            #define UINTPTR_MAX  UINT64_MAX
        #else
            #define INTPTR_MIN   INT32_MIN
            #define INTPTR_MAX   INT32_MAX
            #define UINTPTR_MAX  UINT32_MAX
        #endif

        #define INTMAX_MIN       INT64_MIN
        #define INTMAX_MAX       INT64_MAX
        #define UINTMAX_MAX      UINT64_MAX

        #define PTRDIFF_MIN      INTPTR_MIN
        #define PTRDIFF_MAX      INTPTR_MAX

        #ifndef SIZE_MAX
            #ifdef _WIN64
                #define SIZE_MAX 0xffffffffffffffffui64
            #else
                #define SIZE_MAX 0xffffffffui32
            #endif
        #endif
    ".lines().map(|line| line.trim());

    let temp_dir = tempfile::tempdir()
        .expect("failed to create temp dir");
    let header_path = temp_dir.path().join("dawn.h");
    let output_path = temp_dir.path().join("bindings.rs");

    let header = []
        .into_iter()
        .chain(header_stdint)
        .chain(header_main)
        .collect::<Vec<_>>()
        .join("\n");
    std::fs::write(&header_path, &header)
        .unwrap_or_else(|err| panic!("failed to write file {path}: {err}"));

    bindgen::builder()
        .header(header_path.display().to_string())
        .use_core()
        .derive_default(true)
        .derive_eq(true)
        .derive_ord(true)
        .derive_hash(true)
        .default_enum_style(bindgen::EnumVariation::NewType {
            is_bitfield: false,
            is_global: false,
        })
        .generate()
        .expect("failed to generate bindings for dawn.h")
        .write_to_file(output_path.display().to_string())
        .unwrap_or_else(|err| panic!("failed to write file {path}: {err}"));

    let bindings = std::fs::read_to_string(output_path)
        .unwrap_or_else(|err| panic!("failed to read file {path}: {err}"))
        .lines()
        .filter(|line| {
            !line.starts_with("pub type int") &&
            !line.starts_with("pub type uint")
        })
        .collect::<Vec<_>>()
        .join("\n");
    std::fs::write(workspace_path!("dawn-sys/generated/bindings.rs"), bindings)
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

mod common {
    #[macro_export] macro_rules! workspace_path {
        ($path:literal) => {
            concat!(env!("CARGO_MANIFEST_DIR"), "/../", $path)
        };
    }

    pub const E_JSON_EXPECT_STRING: &str =
        "failed to parse dawn.json: string expected";
    pub const E_JSON_EXPECT_OBJECT: &str =
        "failed to parse dawn.json: object expected";
    pub const E_JSON_EXPECT_ARRAY: &str =
        "failed to parse dawn.json: array expected";

    use proc_macro2::TokenStream;
    use quote::quote;

    pub fn generate_section<F>(
        data: &serde_json::Value,
        name: &str,
        generate: F)
    -> proc_macro2::TokenStream
    where
        F: Fn(&Name, &serde_json::Value) -> TokenStream, {
        let data = data.as_object()
            .expect(E_JSON_EXPECT_OBJECT);
        let items = data.iter()
            .filter(|&(_, data)| data["category"].as_str() == Some(name))
            .map(|(name, data)| generate(&Name(name.clone()), data));
        quote!(#(#items)*)
    }

    pub fn generate_section_by_tag<T, F>(
        data: &serde_json::Value,
        name: &str,
        pred: T,
        generate: F)
    -> proc_macro2::TokenStream
    where
        T: Fn(&[&str]) -> bool,
        F: Fn(&Name, &serde_json::Value) -> TokenStream, {
        let data = data.as_object()
            .expect(E_JSON_EXPECT_OBJECT);
        let items = data.iter()
            .filter(|&(_, data)| data["category"].as_str() == Some(name))
            .filter(|&(_, data)| pred(&{
                data["tags"]
                    .as_array()
                    .map_or(Vec::new(), |tags| {
                        tags.iter()
                            .map(|tag| tag.as_str().expect(E_JSON_EXPECT_STRING))
                            .collect::<Vec<_>>()

                    })
            }))
            .map(|(name, data)| generate(&Name(name.clone()), data));
        quote!(#(#items)*)
    }

    pub struct Name(String);
    impl Name {
        pub fn new(name: String) -> Self {
            Self(name)
        }

        pub fn handle_invalid_ident(&self) -> Self {
            if let Some(prefix) = self.0.get(..2) && ["1D", "2D", "3D"].contains(&prefix) {
                Self(format!("D{}{}", &self.0[..1], &self.0[2..]))
            } else {
                Self(self.0.clone())
            }
        }

        pub fn words(&self) -> impl Iterator<Item = &str> {
            self.0.split_whitespace()
        }

        pub fn screaming_snake_case(&self) -> String {
            self.words()
                .map(|word| word.to_ascii_uppercase())
                .collect::<Vec<_>>()
                .join("_")
        }

        pub fn pascal_case(&self) -> String {
            self.words()
                .map(|word| {
                    let first_chaer = word
                        .chars()
                        .next()
                        .expect("unexpected empty word")
                        .to_ascii_uppercase();
                    Some(first_chaer)
                        .into_iter()
                        .chain(word.chars().skip(1))
                        .collect::<String>()})
                .collect::<Vec<_>>()
                .join("")
        }
    }
}

mod dawn_sys {
    use proc_macro2::TokenStream;
    use quote::{
        format_ident,
        quote,
    };

    use crate::common::*;

    pub fn generate_lib(data: &serde_json::Value) -> TokenStream {
        let mut out = quote!();
        out.extend(generate_section(data, "enum", generate_enum));
        out
    }

    fn generate_enum(type_name: &Name, data: &serde_json::Value) -> TokenStream {
        let type_ident = format_ident!("WGPU{}", type_name.pascal_case());
        let items = data["values"]
            .as_array()
            .expect(E_JSON_EXPECT_ARRAY)
            .iter()
            .map(|data| {
                let name = Name::new({
                    data["name"]
                        .as_str()
                        .expect(E_JSON_EXPECT_STRING)
                        .to_owned()
                });
                let ident = format_ident!(
                    "WGPU{}_{}",
                    type_name.pascal_case(),
                    name.pascal_case());
                quote!(pub const #ident: #type_ident = #type_ident::#ident;)
            });
        quote!(#(#items)*)
    }
}
