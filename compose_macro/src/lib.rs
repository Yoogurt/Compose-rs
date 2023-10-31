#![allow(warnings)]

mod attribute_parser;
mod signature_checker;
mod function_params_collector;
mod hash_code_generator;

use proc_macro2::Ident;
use proc_macro::{Span, TokenStream};
use quote::{quote};
use syn::{AngleBracketedGenericArguments, FieldMutability, Fields, GenericArgument, parse_macro_input, Path, PathArguments, PathSegment, Token, TypePath, Visibility};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{ItemFn, ItemStruct};
use syn::parse::ParseStream;
use syn::token::Colon;
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
                compose::foundation::composer::Composer::start_group(#hash);
                {
                    #function_body
                }
                compose::foundation::composer::Composer::end_group(#hash);
            }
    })
    } else {
        (quote! {
         #[inline]
            #function_visibility fn #function_name() {
                compose::foundation::composer::Composer::start_group(#hash);
                {
                    #function_body
                }
                compose::foundation::composer::Composer::end_group(#hash);
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

#[proc_macro_attribute]
pub fn Leak(attribute: TokenStream, struct_token_stream: TokenStream) -> TokenStream {
    let struct_token = struct_token_stream.clone();
    let mut struct_tokens = parse_macro_input!(struct_token as ItemStruct);

    let struct_ident = &struct_tokens.ident;
    let fields = &mut struct_tokens.fields;

    match fields {
        Fields::Named(field_named) => {
            let named = &mut field_named.named;

            let mut punctuated = Punctuated::<PathSegment, Token![::]>::new();
            punctuated.push(PathSegment { ident: Ident::new("crate", Span::call_site().into()), arguments: Default::default() });
            punctuated.push(PathSegment { ident: Ident::new("foundation", Span::call_site().into()), arguments: Default::default() });
            punctuated.push(PathSegment { ident: Ident::new("memory", Span::call_site().into()), arguments: Default::default() });
            punctuated.push(PathSegment { ident: Ident::new("leak_token", Span::call_site().into()), arguments: Default::default() });

            let mut generic_argument_for_leak_object = Punctuated::<GenericArgument, Token![,]>::new();
            generic_argument_for_leak_object.push(GenericArgument::Type(syn::Type::Verbatim(quote! {
                #struct_ident
            })));
            punctuated.push(PathSegment { ident: Ident::new("LeakToken", Span::call_site().into()), arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                colon2_token: None,
                lt_token: Default::default(),
                args: generic_argument_for_leak_object,
                gt_token: Default::default(),
            }) });

            let new_leak_object_field = syn::Field {
                ident: Some(Ident::new("leak_object", Span::call_site().into())),
                vis: Visibility::Inherited,
                attrs: vec![],
                colon_token: Some(Colon { spans: [Span::call_site().into()] }),
                mutability: FieldMutability::None,
                ty: syn::Type::Path(TypePath { qself: None, path: Path {
                    leading_colon: None,
                    segments: punctuated,
                } }),
            };

            dbg!(&new_leak_object_field);
            named.insert(named.len(), new_leak_object_field);
        }
        _ => {}
    }

    let struct_name = struct_ident.to_string();

    (quote! {
        #struct_tokens

        impl crate::foundation::memory::leak_token::LeakableObject for #struct_ident {
            fn tag() -> &'static str{
                #struct_name
            }
        }
    }).into()
}