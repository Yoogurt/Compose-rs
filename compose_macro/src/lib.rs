#![allow(warnings)]

use proc_macro::{Span, TokenStream, TokenTree};
use std::collections::HashSet;
use std::process::id;
use lazy_static::lazy_static;
use quote::{quote, ToTokens};
use rand::random;
use syn::*;
use std::sync::RwLock;
use syn::spanned::Spanned;

lazy_static! {
    static ref COMPOSER_HASH : RwLock<HashSet<i64>> = RwLock::new(HashSet::new());
}

#[proc_macro_attribute]
pub fn Compose(attribute: TokenStream, funtion: TokenStream) -> TokenStream {
    let function = parse_macro_input!(funtion as ItemFn);
    // dbg!(&tokens);
    let attribute = attribute.into_iter().map_while(|value| {
        match value {
            TokenTree::Ident(ident) => {
                ident.span().source_text()
            }
            _ => {
                None
            }
        }
    }).collect::<HashSet<String>>();

    let mutable_composer_export = if attribute.contains(&"Mutable".to_owned()) {
        quote! {
            let current_composer: &mut compose::foundation::Composer = current_composer;
        }
    } else {
        quote! {
            let current_composer: & compose::foundation::Composer = current_composer;
        }
    };

    let signature = &function.sig;

    if let Some(_) = signature.asyncness {
        let error = syn::Error::new_spanned(&function.sig.asyncness,
                                            "Compose function can not be async");
        return error.to_compile_error().into();
    }
    if !signature.generics.params.is_empty() {
        let error = syn::Error::new_spanned(&signature.generics,
                                            "Compose function can not have generics types");
        return error.to_compile_error().into();
    }
    if signature.receiver().is_some() {
        let error = syn::Error::new_spanned(&signature.receiver(),
                                            "Compose function can not have receiver");
        return error.to_compile_error().into();
    }

    let function_return = &signature.output;
    if function_return != &ReturnType::Default {
        let error = syn::Error::new_spanned(function_return,
                                            "Compose function should be return nothing");
        return error.to_compile_error().into();
    }
    let function_input = &signature.inputs;
    let function_visibility = function.vis;

    let function_in_params = {
        let mut error = Option::<Error>::None;
        let result = function_input.iter().filter_map(|arg| {
            if error.is_some() {
                return None;
            }

            match arg {
                FnArg::Typed(pat) => {
                    match pat.ty.as_ref() {
                        Type::Path(_) | Type::BareFn(_) => {}
                        _ => {
                            error = Some(syn::Error::new_spanned(pat.ty.as_ref(),
                                                                 "Compose function should own params"));
                        }
                    }

                    match pat.pat.as_ref() {
                        Pat::Ident(ident) => {
                            Some(&ident.ident)
                        }
                        _ => {
                            None
                        }
                    }
                }
                FnArg::Receiver(_) => {
                    None
                }
            }
        }).collect::<Vec<_>>();

        match error {
            Some(error) => {
                return error.into_compile_error().into();
            }
            _ => {
                result
            }
        }
    };

    let origin_function_name = &signature.ident;
    let function_name = Ident::new(&format!("__{}__symmetric__", origin_function_name), Span::mixed_site().into());

    let mut hash: i64 = random();

    {
        let mut composer_hash = COMPOSER_HASH.write().unwrap();

        while composer_hash.contains(&hash) {
            hash = random();
        };
        composer_hash.insert(hash.clone());
    }

    let function_body = function.block.as_ref();

    dbg!(&mutable_composer_export);
    let wrapped_function = if !function_in_params.is_empty() {
        (quote! {
         #[inline]
            #function_visibility fn #function_name(#function_input, current_composer: &mut compose::foundation::Composer) {
                current_composer.begin_group(#hash);
                {
                    #mutable_composer_export
                    #function_body
                }
                current_composer.end_group(#hash);
            }
    })
    } else {
        (quote! {
         #[inline]
            #function_visibility fn #function_name(current_composer: &mut compose::foundation::Composer) {
                current_composer.begin_group(#hash);
                {
                    #mutable_composer_export
                    #function_body
                }
                current_composer.end_group(#hash);
            }
    })
    };

    if !function_input.is_empty() {
        (quote! {
        #[inline]
        #function_visibility fn #origin_function_name(#function_input) {
            #wrapped_function

            let mut __composer = compose::foundation::Composer::default();
            #function_name(#(#function_in_params),*, &mut __composer);
        }
    })
    } else {
        (quote! {
        #[inline]
        #function_visibility fn #origin_function_name() {
            #wrapped_function

            let mut __composer = compose::foundation::Composer::default();
            #function_name(&mut __composer);
        }
    })
    }.into()
}
