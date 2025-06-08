extern crate proc_macro;
use syn::*;
use quote::quote;
use quote::format_ident;

#[proc_macro_attribute] pub fn gpu_object(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput {
        attrs,
        vis,
        ident,
        generics: Generics {
            lt_token: None,
            params: generics,
            gt_token: None,
            where_clause: None,
        },
        data: Data::Struct(DataStruct {
            struct_token: _,
            fields: Fields::Unit,
            semi_token: _,
        }),
    } = parse_macro_input!(item as DeriveInput) else {
        panic!("unexpected ast");
    };

    assert!(generics.is_empty(), "generic parameters are not allowed");

    let name = ident
        .to_string()
        .strip_prefix("Gpu")
        .expect("type name must start with 'Gpu'")
        .to_owned();
    let ident_sys = format_ident!("WGPU{name}Impl");
    let ident_sys_add_ref = format_ident!("wgpu{name}AddRef");
    let ident_sys_release = format_ident!("wgpu{name}Release");
    let ident_sys_set_label = format_ident!("wgpu{name}SetLabel");

    let set_label =
        if attr.to_string().as_str() == "labeled" {
            quote!(impl #ident {
                pub fn set_label(&self, label: &str) {
                    unsafe { #ident_sys_set_label(self.ptr(), label.into()) };
                }
            })
        } else if attr.is_empty() {
            quote!()
        } else {
            panic!("only 'gpu_object' and 'gpu_object(labeled)' are allowed");
        };

    quote! {
        #(#attrs)*
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        #[repr(transparent)]
        #vis struct #ident(::core::ptr::NonNull<::dawn_sys::#ident_sys>);

        impl Clone for #ident {
            fn clone(&self) -> Self {
                unsafe { #ident_sys_add_ref(self.ptr()) };
                Self(self.0)
            }
        }

        impl Drop for #ident {
            fn drop(&mut self) {
                unsafe { #ident_sys_release(self.ptr()) };
            }
        }

        impl #ident {
            /// Encapsulate a non-null `dawn-sys` pointer into a `dawn-rs` object.
            /// 
            /// See crate-level documentation for safety notes.
            pub unsafe fn from_ptr(ptr: ::core::ptr::NonNull<#ident_sys>) -> Self {
                Self(ptr)
            }

            /// The underlying `dawn-sys` pointer of this GPU object. The returned
            /// pointer is guaranteed to be non-null.
            /// 
            /// See crate-level documentation for safety notes.
            pub unsafe fn ptr(&self) -> *mut #ident_sys {
                self.0.as_ptr()
            }
        }

        #set_label
    }.into()
}
