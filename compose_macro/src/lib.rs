#![allow(warnings)]

mod attribute_parser;
mod signature_checker;
mod function_params_collector;
mod hash_code_generator;

use proc_macro::{Span, TokenStream, TokenTree};
use std::collections::HashSet;
use std::fmt::format;
use std::process::id;
use lazy_static::lazy_static;
use quote::{quote, ToTokens};
use rand::random;
use syn::*;
use std::sync::RwLock;
use syn::spanned::Spanned;
use crate::attribute_parser::parse_attribute;
use crate::signature_checker::verify_signature;
use crate::function_params_collector::collect_function_params;
use crate::hash_code_generator::generate_hash_code;

#[proc_macro_attribute]
pub fn Compose(attribute: TokenStream, funtion: TokenStream) -> TokenStream {
    let function = parse_macro_input!(funtion as ItemFn);
    let attribute = parse_attribute(attribute);

    let mutable_composer_export =
        attribute.compose_mutable_descriptor();

    let signature = &function.sig;
    match verify_signature(signature) {
        Err(token_stream) => {
            return token_stream;
        }
        _ => {}
    }

    let function_inputs_with_type = &signature.inputs;
    let function_inputs = match collect_function_params(function_inputs_with_type) {
        Err(error) => {
            return error;
        }
        Ok(result) => { result }
    };
    let function_visibility = function.vis;

    let origin_function_name = &signature.ident;
    let function_name = Ident::new(&format!("__{}__compose_synthesis__", origin_function_name), Span::mixed_site().into());
    let function_body = function.block.as_ref();

    let hash = generate_hash_code();

    let wrapped_function = if !function_inputs.is_empty() {
        (quote! {
         #[inline]
            #function_visibility fn #function_name(#function_inputs_with_type) {
                compose::foundation::Composer::begin_group(#hash);
                {
                    #function_body
                }
                compose::foundation::Composer::end_group(#hash);
            }
    })
    } else {
        (quote! {
         #[inline]
            #function_visibility fn #function_name() {
                compose::foundation::Composer::begin_group(#hash);
                {
                    #function_body
                }
                compose::foundation::Composer::end_group(#hash);
            }
    })
    };

    let result = if !function_inputs.is_empty() {
        (quote! {
        #[inline]
        #function_visibility fn #origin_function_name(#function_inputs_with_type) {
            #wrapped_function

            #function_name(#(#function_inputs),*);
        }
    })
    } else {
        (quote! {
        #[inline]
        #function_visibility fn #origin_function_name() {
            #wrapped_function

            #function_name();
        }
    })
    };

    result.into()
}
