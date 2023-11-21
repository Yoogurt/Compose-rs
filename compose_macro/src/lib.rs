#![allow(warnings)]

use proc_macro::{Span, TokenStream};

use proc_macro2::Ident;
use quote::quote;
use syn::ItemFn;
use syn::parse_macro_input;
use syn::ReturnType;
use syn::spanned::Spanned;

use crate::attribute_parser::parse_attribute;
use crate::function_params_collector::collect_function_params;
use crate::hash_code_generator::generate_hash_code;
use crate::signature_checker::verify_signature;

mod attribute_parser;
mod function_params_collector;
mod hash_code_generator;
mod signature_checker;

#[proc_macro_attribute]
pub fn Composable(attribute: TokenStream, function: TokenStream) -> TokenStream {
    let function = parse_macro_input!(function as ItemFn);
    let attribute = parse_attribute(attribute);

    let mutable_composer_export = attribute.compose_mutable_descriptor();

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
        Ok(result) => result,
    };
    let function_visibility = function.vis;

    let origin_function_name = &signature.ident;
    let function_name = Ident::new(
        &format!("__{}__compose_synthesis__", origin_function_name),
        Span::mixed_site().into(),
    );
    let function_body = function.block.as_ref();

    let hash = generate_hash_code();

    let function_generics = &function.sig.generics;
    let where_calcause = function_generics.where_clause.as_ref();
    let output = &function.sig.output;

    let start_group_stat = match output {
        ReturnType::Default => {
            quote! {
                 compose::foundation::composer::Composer::start_group(#hash);
                 compose::foundation::composer::Composer::start_restart_group();
            }
        }
        _ => {
            TokenStream::default().into()
        }
    };

    let end_group_stat = match output {
        ReturnType::Default => {
            quote! {
                compose::foundation::composer::Composer::end_restart_group();
                compose::foundation::composer::Composer::end_group(#hash);
            }
        }
        _ => {
            TokenStream::default().into()
        }
    };

    let wrapped_function = if !function_inputs.is_empty() {
        (quote! {
             #[inline]
                #function_visibility fn #function_name #function_generics(#function_inputs_with_type) #output #where_calcause {
                    #start_group_stat
                    let __result__ = { #function_body };
                    #end_group_stat
                    __result__
                }
        })
    } else {
        (quote! {
             #[inline]
                #function_visibility fn #function_name #function_generics() #output #where_calcause {
                    #start_group_stat
                    let __result__ = { #function_body };
                    #end_group_stat
                    __result__
                }
        })
    };

    let result = if !function_inputs.is_empty() {
        (quote! {
            #[inline]
            #function_visibility fn #origin_function_name #function_generics(#function_inputs_with_type) #output #where_calcause {
                #wrapped_function

                #function_name(#(#function_inputs),*)
            }
        })
    } else {
        (quote! {
            #[inline]
            #function_visibility fn #origin_function_name #function_generics() #output #where_calcause {
                #wrapped_function

                #function_name()
            }
        })
    };

    result.into()
}