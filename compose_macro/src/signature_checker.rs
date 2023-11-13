use proc_macro::TokenStream;

use syn::*;

pub(crate) fn verify_signature(signature: &Signature) -> std::result::Result<(), TokenStream> {
    if let Some(_) = signature.asyncness {
        let error =
            syn::Error::new_spanned(&signature.asyncness, "Compose function can not be async");
        return Err(error.to_compile_error().into());
    }
    if signature.receiver().is_some() {
        let error = syn::Error::new_spanned(
            &signature.receiver(),
            "Compose function can not have receiver",
        );
        return Err(error.to_compile_error().into());
    }

    // let function_return = &signature.output;
    // if function_return != &ReturnType::Default {
    //     let error =
    //         syn::Error::new_spanned(function_return, "Compose function should be return nothing");
    //     return Err(error.to_compile_error().into());
    // }

    Ok(())
}
