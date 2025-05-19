#[macro_export] macro_rules! workspace_path {
    ($path:literal) => {
        concat!(env!("CARGO_MANIFEST_DIR"), "/../", $path)
    };
}

pub const E_JSON_STRING_EXPECTED: &str = "failed to parse dawn.json: string expected";
pub const E_JSON_OBJECT_EXPECTED: &str = "failed to parse dawn.json: object expected";
pub const E_JSON_ARRAY_EXPECTED: &str = "failed to parse dawn.json: array expected";

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
        .expect(E_JSON_OBJECT_EXPECTED);
    let items = data.iter()
        .filter(|&(_, data)| data["category"].as_str() == Some(name))
        .map(|(name, data)| generate(&Name(name.clone()), data));
    quote!(#(#items)*)
}

pub struct Name(String);
impl Name {
    pub fn new(name: String) -> Self {
        Self(name)
    }

    pub fn handle_invalid_ident(self) -> Self {
        let Self(ref s) = self;
        if let Some(prefix) = s.get(..2) && ["1D", "2D", "3D"].contains(&prefix) {
            Self(format!("D{}{}", &s[..1], &s[2..]))
        } else {
            self
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
