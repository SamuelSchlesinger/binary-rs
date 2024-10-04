extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(Binary)]
pub fn derive_binary(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);
    let ty_name = &input.ident;
    match input.data {
        Data::Struct(s) => {
            match s.fields {
                Fields::Named(fields) => {
                    let parse_code = fields.named.iter().map(|field| {
                        let field_ident = &field.ident;
                        let field_ty = &field.ty;
                        quote! {
                            let (#field_ident, bs) = #field_ty::parse(bs)?;
                        }
                    });
                    let field_names = fields
                        .named
                        .iter()
                        .map(|field| &field.ident)
                        .collect::<Vec<_>>();
                    let unparse_code = fields.named.iter().map(|field| {
                        let field_ident = &field.ident;
                        quote! {
                            #field_ident.unparse(bs);
                        }
                    });
                    quote! {
                        impl Binary for #ty_name {
                            fn parse(bs: &[u8]) -> Option<(Self, &[u8])> {
                                #(#parse_code);*
                                Some((#ty_name { #(#field_names),* }, bs))
                            }

                            fn unparse(&self, bs: &mut Vec<u8>) {
                                let #ty_name { #(#field_names),* } = &self;
                                #(#unparse_code);*
                            }
                        }
                    }
                    .into()
                }
                Fields::Unnamed(fields) => {
                    let field_idents = {
                        let mut v = Vec::new();
                        for i in 0..fields.unnamed.len() {
                            v.push(Ident::new(&format!("field_{}", i), Span::call_site()));
                        }
                        v
                    };
                    let parse_code = fields.unnamed.iter().zip(field_idents.iter()).map(
                        |(field, field_ident)| {
                            let field_ty = &field.ty;
                            quote! {
                                let (#field_ident, bs) = #field_ty::parse(bs)?;
                            }
                        },
                    );
                    let unparse_code = field_idents.iter().map(|field_ident| {
                        quote! {
                            #field_ident.unparse(bs);
                        }
                    });
                    quote! {
                        impl Binary for #ty_name {
                            fn parse(bs: &[u8]) -> Option<(Self, &[u8])> {
                                #(#parse_code);*
                                Some((#ty_name ( #(#field_idents),* ), bs))
                            }

                            fn unparse(&self, bs: &mut Vec<u8>) {
                                let #ty_name (#(#field_idents),*) = &self;
                                #(#unparse_code);*
                            }
                        }
                    }
                    .into()
                }
                Fields::Unit => quote! {
                    impl Binary for #ty_name {
                        fn parse(bs: &[u8]) -> Option<(Self, &[u8])> {
                            return Some((#ty_name, bs));
                        }

                        fn unparse(&self, bs: &mut Vec<u8>) {}
                    }
                }
                .into(),
            }
        }
        Data::Enum(e) => {
            // supports enums of up to 256 variants
            if e.variants.len() > 256 {
                return quote! { compile_error!("more than 256 variants") }.into();
            }
            let parse_match_branches = e.variants.iter().zip(0u8..).map(|(variant, tag)| {
                let variant_ident = &variant.ident;
                match &variant.fields {
                    Fields::Named(fields) => {
                        let parse_code = fields.named.iter().map(|field| {
                            let field_ident = &field.ident;
                            let field_ty = &field.ty;
                            quote! {
                                let (#field_ident, bs) = #field_ty::parse(bs)?;
                            }
                        });
                        let field_names = fields
                            .named
                            .iter()
                            .map(|field| &field.ident)
                            .collect::<Vec<_>>();
                        quote! {
                            #tag => {
                                #(#parse_code);*
                                Some((#ty_name::#variant_ident { #(#field_names),* }, bs))
                            }
                        }
                    }
                    Fields::Unnamed(fields) => {
                        let field_idents = {
                            let mut v = Vec::new();
                            for i in 0..fields.unnamed.len() {
                                v.push(Ident::new(&format!("field_{}", i), Span::call_site()));
                            }
                            v
                        };
                        let parse_code = fields.unnamed.iter().zip(field_idents.iter()).map(
                            |(field, field_ident)| {
                                let field_ty = &field.ty;
                                quote! {
                                    let (#field_ident, bs) = #field_ty::parse(bs)?;
                                }
                            },
                        );
                        quote! {
                            #tag => {
                                #(#parse_code);*
                                Some((#ty_name::#variant_ident ( #(#field_idents),* ), bs))
                            }
                        }
                    }
                    Fields::Unit => {
                        quote! {
                            #tag => {
                                return Some((#ty_name::#variant_ident, bs));
                            }
                        }
                    }
                }
            });
            let unparse_match_branches = e.variants.iter().zip(0u8..).map(|(variant, tag)| {
                let variant_ident = &variant.ident;
                match &variant.fields {
                    Fields::Named(fields) => {
                        let unparse_code = fields.named.iter().map(|field| {
                            let field_ident = &field.ident;
                            quote! {
                                #field_ident.unparse(bs);
                            }
                        });
                        let field_names = fields
                            .named
                            .iter()
                            .map(|field| &field.ident)
                            .collect::<Vec<_>>();
                        quote! {
                            #ty_name::#variant_ident { #(#field_names),* } => {
                                bs.push(#tag);
                                #(#unparse_code);*
                            }
                        }
                    }
                    Fields::Unnamed(fields) => {
                        let field_idents = {
                            let mut v = Vec::new();
                            for i in 0..fields.unnamed.len() {
                                v.push(Ident::new(&format!("field_{}", i), Span::call_site()));
                            }
                            v
                        };
                        let unparse_code = fields.unnamed.iter().zip(field_idents.iter()).map(
                            |(_field, field_ident)| {
                                quote! {
                                    #field_ident.unparse(bs);
                                }
                            },
                        );
                        quote! {
                            #ty_name::#variant_ident (#(#field_idents),*) => {
                                bs.push(#tag);
                                #(#unparse_code);*
                            }
                        }
                    }
                    Fields::Unit => {
                        quote! {
                            #ty_name::#variant_ident => {
                                bs.push(#tag);
                            }
                        }
                    }
                }
            });
            quote! {
                impl Binary for #ty_name {
                    fn parse(bs: &[u8]) -> Option<(Self, &[u8])> {
                        if bs.len() == 0 {
                            return None;
                        }
                        let b = bs[0];
                        let bs = &bs[1..];
                        match b {
                            #(#parse_match_branches)*
                            _ => None
                        }
                    }

                    fn unparse(&self, bs: &mut Vec<u8>) {
                        match self {
                            #(#unparse_match_branches)*
                        }
                    }
                }
            }
            .into()
        }
        _ => quote! { compile_error!("Binary can only be derived on structs and enums") }.into(),
    }
}
