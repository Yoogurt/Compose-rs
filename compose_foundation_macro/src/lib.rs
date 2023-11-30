use proc_macro::{Span, TokenStream};
use std::collections::HashMap;

use modifier_element_trait_generator::*;
use proc_macro2::Ident;
use quote::{quote, TokenStreamExt, ToTokens};
use syn::{AngleBracketedGenericArguments, FieldMutability, Fields, GenericArgument, Meta, parse_macro_input, Path, PathArguments, PathSegment, Token, TypePath, Visibility};
use syn::ItemStruct;
use syn::punctuated::Punctuated;
use syn::token::Colon;

mod modifier_element_trait_generator;

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
            punctuated.push(PathSegment {
                ident: Ident::new("crate", Span::call_site().into()),
                arguments: Default::default(),
            });
            punctuated.push(PathSegment {
                ident: Ident::new("foundation", Span::call_site().into()),
                arguments: Default::default(),
            });
            punctuated.push(PathSegment {
                ident: Ident::new("memory", Span::call_site().into()),
                arguments: Default::default(),
            });
            punctuated.push(PathSegment {
                ident: Ident::new("leak_token", Span::call_site().into()),
                arguments: Default::default(),
            });

            let mut generic_argument_for_leak_object =
                Punctuated::<GenericArgument, Token![,]>::new();
            generic_argument_for_leak_object.push(GenericArgument::Type(syn::Type::Verbatim(
                quote! {
                    #struct_ident
                },
            )));
            punctuated.push(PathSegment {
                ident: Ident::new("LeakToken", Span::call_site().into()),
                arguments: PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                    colon2_token: None,
                    lt_token: Default::default(),
                    args: generic_argument_for_leak_object,
                    gt_token: Default::default(),
                }),
            });

            let new_leak_object_field = syn::Field {
                ident: Some(Ident::new("leak_object", Span::call_site().into())),
                vis: Visibility::Inherited,
                attrs: vec![],
                colon_token: Some(Colon {
                    spans: [Span::call_site().into()],
                }),
                mutability: FieldMutability::None,
                ty: syn::Type::Path(TypePath {
                    qself: None,
                    path: Path {
                        leading_colon: None,
                        segments: punctuated,
                    },
                }),
            };

            // dbg!(&new_leak_object_field);
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
    })
        .into()
}

#[proc_macro_derive(ModifierElement, attributes(Impl))]
pub fn ModifierElement(struct_token_stream: TokenStream) -> TokenStream {
    let struct_tokens = parse_macro_input!(struct_token_stream as ItemStruct);
    let struct_ident = struct_tokens.ident.clone();

    let converter = [
        ("LayoutModifierNodeConverter", "as_layout_modifier_node", "as_layout_modifier_node_mut", "LayoutModifierNode"),
        ("DrawModifierNodeConverter", "as_draw_modifier_node", "as_draw_modifier_node_mut", "DrawModifierNode"),
        ("ParentDataModifierNodeConverter", "as_parent_data_modifier_node", "as_parent_data_modifier_node_mut", "ParentDataModifierNode"),
        ("LayoutAwareModifierNodeConverter", "as_layout_aware_modifier_node", "as_layout_aware_modifier_node_mut", "LayoutAwareModifierNode"),
    ];

    let mut mapping = converter.into_iter().map(|value| (value.0, (value.1, value.2, value.3, false))).collect::<HashMap<&str, (&str, &str, &str, bool)>>();

    for attribute in struct_tokens.attrs {
        if let Meta::List(list) = attribute.meta {
            list.tokens.into_iter().for_each(|token| {
                if let proc_macro2::TokenTree::Ident(ident) = token {
                    if let Some(do_generate) = mapping.get_mut(ident.to_string().as_str()) {
                        do_generate.3 = true;
                    }
                }
            });
        } else {
            return syn::Error::new(
                Span::call_site().into(),
                "Wrong type on attribute Impl, expected Meta::List",
            ).to_compile_error().into();
        }
    }

    let any_converter = generate_any_converter(&struct_ident);
    let modifier_element = generate_modifier_element(&struct_ident);

    let mut token_stream = quote! {
        #any_converter
        #modifier_element
    };

    _ = mapping.into_iter().for_each(|converter| {
        let converter_ident = Ident::new(converter.0, Span::call_site().into());
        let as_ref = Ident::new(converter.1.0, Span::call_site().into());
        let as_mut = Ident::new(converter.1.1, Span::call_site().into());
        let ret_ident = Ident::new(converter.1.2, Span::call_site().into());

        generate_converter(&struct_ident, converter_ident, as_ref, as_mut, ret_ident, converter.1.3).to_tokens(&mut token_stream)
    });


    token_stream.into()
}


#[proc_macro_derive(AnyConverter)]
pub fn AnyConverter(struct_token_stream: TokenStream) -> TokenStream {
    let mut struct_tokens = parse_macro_input!(struct_token_stream as ItemStruct);
    let struct_ident = struct_tokens.ident.clone();

    let any_converter = generate_any_converter(&struct_ident);

    (quote! {
        #any_converter
    }).into()
}