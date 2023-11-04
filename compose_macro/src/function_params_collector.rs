use proc_macro::TokenStream;
use std::env::Args;
use std::result::Result as STDResult;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::*;

pub(crate) fn collect_function_params(
    function_input: &Punctuated<FnArg, Comma>,
) -> STDResult<Vec<&Ident>, TokenStream> {
    let mut error = Option::<Error>::None;
    let result = function_input
        .iter()
        .filter_map(|arg| {
            if error.is_some() {
                return None;
            }

            match arg {
                FnArg::Typed(pat) => {
                    match pat.ty.as_ref() {
                        Type::Path(_) | Type::BareFn(_) | Type::ImplTrait(_) => {}
                        _ => {
                            error = Some(syn::Error::new_spanned(
                                pat.ty.as_ref(),
                                "Compose function should own params",
                            ));
                        }
                    }

                    match pat.pat.as_ref() {
                        Pat::Ident(ident) => Some(&ident.ident),
                        _ => None,
                    }
                }
                FnArg::Receiver(_) => None,
            }
        })
        .collect::<Vec<_>>();

    match error {
        Some(error) => {
            return Err(error.into_compile_error().into());
        }
        _ => {
            return Ok(result);
        }
    }
}
