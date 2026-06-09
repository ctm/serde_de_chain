//! Derive macro that lets a type be deserialized from its current
//! representation or, if that fails, from a previous representation.
//!
//! Apply `#[derive(SerdeDeChain)]` together with
//! `#[serde_de_chain(OldType)]` to a struct or enum and provide
//! `impl From<OldType> for Self`. Deserialization will first try the
//! current representation; on failure it falls back to `OldType`'s
//! `Deserialize` implementation. If `OldType` itself derives
//! `SerdeDeChain`, the chain extends transparently.
//!
//! See `tests/integration.rs` for end-to-end examples.

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{
    Attribute, Data, DataEnum, DataStruct, DeriveInput, Fields, Ident, Type, parse_macro_input,
};

#[proc_macro_derive(SerdeDeChain, attributes(serde_de_chain))]
pub fn derive_serde_de_chain(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn expand(input: DeriveInput) -> syn::Result<TokenStream2> {
    let name = &input.ident;
    let old_type = parse_old_type(&input)?;
    let serde_attrs: Vec<&Attribute> = input
        .attrs
        .iter()
        .filter(|a| a.path().is_ident("serde"))
        .collect();
    let shadow_ident = format_ident!("__SerdeDeChainShadow");

    let (shadow_def, conversion) = match &input.data {
        Data::Enum(data) => (
            enum_shadow(data, &serde_attrs, &shadow_ident),
            enum_conversion(data, &shadow_ident),
        ),
        Data::Struct(data) => (
            struct_shadow(data, &serde_attrs, &shadow_ident),
            struct_conversion(data),
        ),
        Data::Union(_) => {
            return Err(syn::Error::new_spanned(
                &input.ident,
                "SerdeDeChain does not support unions",
            ));
        }
    };

    Ok(quote! {
        #[automatically_derived]
        impl<'de> ::serde::Deserialize<'de> for #name {
            fn deserialize<__D>(
                deserializer: __D,
            ) -> ::core::result::Result<Self, <__D as ::serde::Deserializer<'de>>::Error>
            where
                __D: ::serde::Deserializer<'de>,
            {
                #shadow_def

                #[derive(::serde::Deserialize)]
                #[serde(untagged)]
                enum __SerdeDeChainNewOld {
                    New(#shadow_ident),
                    Old(#old_type),
                }

                <__SerdeDeChainNewOld as ::serde::Deserialize>::deserialize(deserializer).map(
                    |__v| match __v {
                        __SerdeDeChainNewOld::New(new) => #conversion,
                        __SerdeDeChainNewOld::Old(old) => ::core::convert::From::from(old),
                    },
                )
            }
        }
    })
}

fn parse_old_type(input: &DeriveInput) -> syn::Result<Type> {
    let mut found: Option<Type> = None;
    for attr in &input.attrs {
        if attr.path().is_ident("serde_de_chain") {
            if found.is_some() {
                return Err(syn::Error::new_spanned(
                    attr,
                    "duplicate #[serde_de_chain(...)] attribute",
                ));
            }
            found = Some(attr.parse_args::<Type>()?);
        }
    }
    found.ok_or_else(|| {
        syn::Error::new_spanned(
            &input.ident,
            "SerdeDeChain requires #[serde_de_chain(OldType)]",
        )
    })
}

fn enum_shadow(data: &DataEnum, serde_attrs: &[&Attribute], shadow: &Ident) -> TokenStream2 {
    let variants = &data.variants;
    quote! {
        #[derive(::serde::Deserialize)]
        #(#serde_attrs)*
        enum #shadow {
            #variants
        }
    }
}

fn struct_shadow(data: &DataStruct, serde_attrs: &[&Attribute], shadow: &Ident) -> TokenStream2 {
    match &data.fields {
        Fields::Named(fields) => quote! {
            #[derive(::serde::Deserialize)]
            #(#serde_attrs)*
            struct #shadow #fields
        },
        Fields::Unnamed(fields) => quote! {
            #[derive(::serde::Deserialize)]
            #(#serde_attrs)*
            struct #shadow #fields;
        },
        Fields::Unit => quote! {
            #[derive(::serde::Deserialize)]
            #(#serde_attrs)*
            struct #shadow;
        },
    }
}

fn enum_conversion(data: &DataEnum, shadow: &Ident) -> TokenStream2 {
    let arms = data.variants.iter().map(|v| {
        let variant = &v.ident;
        match &v.fields {
            Fields::Named(fields) => {
                let names: Vec<&Ident> = fields
                    .named
                    .iter()
                    .map(|f| f.ident.as_ref().unwrap())
                    .collect();
                quote! {
                    #shadow::#variant { #(#names),* } => Self::#variant { #(#names),* }
                }
            }
            Fields::Unnamed(fields) => {
                let binds: Vec<Ident> = (0..fields.unnamed.len())
                    .map(|i| format_ident!("__f{}", i))
                    .collect();
                quote! {
                    #shadow::#variant(#(#binds),*) => Self::#variant(#(#binds),*)
                }
            }
            Fields::Unit => quote! {
                #shadow::#variant => Self::#variant
            },
        }
    });
    quote! {
        match new {
            #(#arms,)*
        }
    }
}

fn struct_conversion(data: &DataStruct) -> TokenStream2 {
    match &data.fields {
        Fields::Named(fields) => {
            let names: Vec<&Ident> = fields
                .named
                .iter()
                .map(|f| f.ident.as_ref().unwrap())
                .collect();
            quote! {
                Self { #(#names: new.#names),* }
            }
        }
        Fields::Unnamed(fields) => {
            let indices: Vec<syn::Index> = (0..fields.unnamed.len()).map(syn::Index::from).collect();
            quote! {
                Self(#(new.#indices),*)
            }
        }
        Fields::Unit => quote! { Self },
    }
}
