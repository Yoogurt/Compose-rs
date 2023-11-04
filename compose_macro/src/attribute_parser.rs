use proc_macro::{TokenStream, TokenTree};
use quote::quote;
use std::collections::HashSet;
use syn::*;

pub(crate) struct ComposeAttribute {
    pub(crate) mutable: bool,
}

impl ComposeAttribute {
    pub(crate) fn compose_mutable_descriptor(&self) -> proc_macro2::TokenStream {
        if self.mutable {
            quote! {
                let current_composer: &mut compose::foundation::Composer = current_composer;
            }
        } else {
            quote! {
                let current_composer: &compose::foundation::Composer = current_composer;
            }
        }
    }
}

pub(crate) fn parse_attribute(attribute: TokenStream) -> ComposeAttribute {
    let attribute = attribute
        .into_iter()
        .map_while(|value| match value {
            TokenTree::Ident(ident) => ident.span().source_text(),
            _ => None,
        })
        .collect::<HashSet<String>>();

    let mut mutable = attribute.contains(&"Mutable".to_owned());

    ComposeAttribute { mutable: mutable }
}
