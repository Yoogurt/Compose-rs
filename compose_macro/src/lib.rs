#![allow(warnings)]

use proc_macro::{Span, TokenStream};
use std::process::id;
use lazy_static::lazy_static;
use quote::{quote, ToTokens};
use rand::random;
use syn::*;
use std::sync::RwLock;
use syn::spanned::Spanned;

lazy_static! {
    static ref COMPOSER_HASH : RwLock<Vec<i64>> = RwLock::new(vec![]);
}

#[proc_macro_attribute]
pub fn Compose(attribute: TokenStream, funtion: TokenStream) -> TokenStream {
    let tokens = parse_macro_input!(funtion as ItemFn);
    dbg!(&tokens);

    let signature = &tokens.sig;

    if let Some(_) = signature.asyncness {
        let error = syn::Error::new_spanned(&tokens.sig.asyncness,
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

    let function_in_params = {
        let mut error = Option::<Error>::None;
        let result = function_input.iter().filter_map(|arg| {
            if error.is_some() {
                return None;
            }

            match arg {
                FnArg::Typed(pat) => {
                    match pat.ty.as_ref() {
                        Type::Path(_) => {}
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
        composer_hash.push(hash.clone());
    }

    let function_body = tokens.block.as_ref();
    let ret = (quote! {
        #[inline]
        fn #origin_function_name(#function_input) {
            #[inline]
            fn #function_name(#function_input, current_composer: &compose::foundation::Composer) {
                #function_body
            }

            let __composer = compose::foundation::Composer { _hash: #hash };
            #function_name(#(#function_in_params),* , &__composer);
        }
    });

    ret.into()
}
