mod modifier_element_trait_generator;

use modifier_element_trait_generator::*;
use proc_macro::{Span, TokenStream};
use std::collections::HashMap;
use proc_macro2::Ident;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Colon;
use syn::{parse_macro_input, AngleBracketedGenericArguments, FieldMutability, Fields, GenericArgument, Path, PathArguments, PathSegment, Token, TypePath, Visibility, Meta};
use syn::ItemStruct;

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
pub fn ModifierElementTraitImpl(struct_token_stream: TokenStream) -> TokenStream {
    let mut struct_tokens = parse_macro_input!(struct_token_stream as ItemStruct);
    let struct_ident = struct_tokens.ident.clone();

    let mut do_generate_layout_modifier_node_converter = false;
    let mut do_generate_draw_modifier_node_converter = false;

    let mut mapping: HashMap<&'static str, &mut bool> = HashMap::new();
    mapping.insert("LayoutModifierNodeConverter", &mut do_generate_layout_modifier_node_converter);
    mapping.insert("DrawModifierNodeConverter", &mut do_generate_draw_modifier_node_converter);

    for attribute in struct_tokens.attrs {
        if let Meta::List(list) = attribute.meta {
            list.tokens.into_iter().for_each(|token| {
                if let proc_macro2::TokenTree::Ident(ident) = token {
                    dbg!(ident.to_string().as_str());
                    if let Some(do_convert) = mapping.get_mut(ident.to_string().as_str()) {
                        **do_convert = true;
                    }
                }
            });
        } else {
            return syn::Error::new(
                Span::call_site().into(),
                "Compose function should own params",
            ).to_compile_error().into();
        }
    }

    let any_converter = generate_any_converter(&struct_ident);
    let modifier_element = generate_modifier_element(&struct_ident);
    let layout_modifier_node_converter = generate_layout_modifier_node_converter(&struct_ident, do_generate_layout_modifier_node_converter);
    let draw_modifier_node_converter = generate_draw_modifier_node_converter(&struct_ident, do_generate_draw_modifier_node_converter);

    (quote! {
        #modifier_element
        #any_converter
        #layout_modifier_node_converter
        #draw_modifier_node_converter
    }).into()
}