use proc_macro2::TokenStream;
use quote::{quote, TokenStreamExt};
use syn::Ident;

pub(crate) fn generate_modifier_element(struct_ident: &Ident) -> TokenStream {
    (quote! {
          impl crate::foundation::modifier::ModifierElement for #struct_ident {
                fn as_modifier_element(&self) -> &dyn crate::foundation::modifier::ModifierElement { self }
                fn as_modifier_element_mut(&mut self) -> &mut dyn crate::foundation::modifier::ModifierElement { self }
        }
    }).into()
}

pub(crate) fn generate_any_converter(struct_ident: &Ident) -> TokenStream {
    (quote! {
         impl crate::foundation::oop::AnyConverter for #struct_ident {
            fn as_any(&self) -> &dyn std::any::Any {
                self
            }

            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }
        }
    }).into()
}

pub(crate) fn generate_delegate_type(struct_ident: &Ident) -> TokenStream {
    (quote! {
            impl crate::foundation::delegatable_node::DelegatableNode for #struct_ident {
                fn get_node(&self) -> crate::foundation::delegatable_node::DelegatableKind {
                    crate::foundation::delegatable_node::DelegatableKind::This
                }
            }
    }).into()
}

pub(crate) fn generate_node_patch(struct_ident: &Ident,
                                  node_patchs: Vec<Ident>) -> TokenStream {
    dbg!(format!("node_path_impl {:?} is impl for {}", node_patchs, struct_ident.to_string()).as_str());

    let mut node_patch_token_stream = TokenStream::default();
    node_patch_token_stream.append_separated(node_patchs.into_iter().map(|node_patch| quote!({ crate::foundation::modifier::NodeKind::#node_patch })), quote!({ | }));

    (quote! {
            impl crate::foundation::modifier::NodeKindPatch for #struct_ident {
                fn get_node_kind(&self) -> u32 {
                    (#node_patch_token_stream).into()
                }
            }
    }).into()
}

pub(crate) fn generate_converter(struct_ident: &Ident,
                                 converter_ident: Ident,
                                 as_ref: Ident,
                                 as_mut: Ident,
                                 ret_ident: Ident,
                                 generate: bool) -> TokenStream {
    if !generate {
        (quote! {
          impl crate::foundation::oop::#converter_ident for #struct_ident {}
        }).into()
    } else {
        dbg!(format!("{:?} is impl for {}", converter_ident, struct_ident.to_string()).as_str());

        (quote! {
            impl crate::foundation::oop::#converter_ident for #struct_ident {
                fn #as_ref(&self) -> Option<&dyn crate::foundation::modifier_node::#ret_ident> {
                    Some(self)
                }

                fn #as_mut(&mut self) -> Option<&mut dyn crate::foundation::modifier_node::#ret_ident> {
                    Some(self)
                }
            }
    }).into()
    }
}