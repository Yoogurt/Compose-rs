use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident};

pub(crate) fn generate_modifier_element(struct_ident: &Ident) -> TokenStream {
    (quote! {
          impl crate::foundation::modifier::ModifierElement for #struct_ident {}
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

pub(crate) fn generate_layout_modifier_node_converter(struct_ident: &Ident, generate: bool) -> TokenStream {
    if !generate {
        (quote! {
          impl crate::foundation::oop::LayoutModifierNodeConverter for #struct_ident {}
        }).into()
    } else {
        dbg!(format!("LayoutModifierNodeConverter is impl for {}", struct_ident.to_string()).as_str());

        (quote! {
            impl crate::foundation::oop::LayoutModifierNodeConverter for #struct_ident {
                fn as_layout_modifier_node(&self) -> Option<&dyn crate::foundation::layout_modifier_node::LayoutModifierNode> {
                    Some(self)
                }

                fn as_layout_modifier_node_mut(&mut self) -> Option<&mut dyn crate::foundation::layout_modifier_node::LayoutModifierNode> {
                    Some(self)
                }
        }
    }).into()
    }
}

pub(crate) fn generate_draw_modifier_node_converter(struct_ident: &Ident, generate: bool) -> TokenStream {
    if !generate {
        (quote! {
            impl crate::foundation::oop::DrawModifierNodeConverter for #struct_ident {}
        }).into()
    } else {
        dbg!(format!("DrawModifierNodeConverter is impl for {}", struct_ident.to_string()).as_str());

        (quote! {
          impl crate::foundation::oop::DrawModifierNodeConverter for #struct_ident {
            fn as_draw_modifier_node(&self) -> Option<&dyn crate::foundation::ui::draw::DrawModifierNode> {
                Some(self)
            }

            fn as_draw_modifier_node_mut(&mut self) -> Option<&mut dyn crate::foundation::ui::draw::DrawModifierNode> {
                Some(self)
            }
        }
    }).into()
    }
}