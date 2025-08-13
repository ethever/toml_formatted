use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

#[proc_macro_derive(TomlDocFormatted, attributes(inline_table))]
pub fn derive_toml_doc(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = &input.ident;

    // Collect names of fields marked #[inline_table]
    let mut inline_fields = Vec::<syn::Ident>::new();

    if let Data::Struct(data_struct) = &input.data {
        if let Fields::Named(named) = &data_struct.fields {
            for f in &named.named {
                let fname = f.ident.clone().unwrap();
                let mut is_inline = false;

                for attr in &f.attrs {
                    // syn v2: use path().is_ident(...)
                    if attr.path().is_ident("inline_table") {
                        is_inline = true;
                        break;
                    }
                }

                if is_inline {
                    inline_fields.push(fname);
                }
            }
        }
    }

    let field_strs: Vec<String> = inline_fields.iter().map(|id| id.to_string()).collect();

    let expanded = quote! {
        impl ::toml_formatted::TomlDocExt for #ident {
            fn to_toml_document_formatted(&self) -> ::toml_formatted::toml_edit::DocumentMut {
                let pretty_string = ::toml_formatted::toml_edit::ser::to_string_pretty(&self).unwrap();
                use std::str::FromStr;

                let mut __doc = ::toml_formatted::toml_edit::DocumentMut::from_str(&pretty_string).unwrap();

                #(
                    let _ = ::toml_formatted::inline_field(&mut __doc, #field_strs);
                )*

                __doc
            }
        }
    };

    expanded.into()
}
