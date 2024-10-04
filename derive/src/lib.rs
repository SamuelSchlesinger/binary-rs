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
                    let field_names = fields.named.iter().map(|field| &field.ident);
                    quote! {
                        impl Binary for #ty_name {
                            fn parse(bs: &[u8]) -> Option<(Self, &[u8])> {
                                #(#parse_code);*
                                Some((#ty_name { #(#field_names),* }, bs))
                            }

                            fn unparse(&self, bs: &mut Vec<u8>) {

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
                    quote! {
                        impl Binary for #ty_name {
                            fn parse(bs: &[u8]) -> Option<(Self, &[u8])> {
                                #(#parse_code);*
                                Some((#ty_name ( #(#field_idents),* ), bs))
                            }

                            fn unparse(&self, bs: &mut Vec<u8>) {

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
        // TODO Data::Enum(e) => unimplemented!()
        _ => quote! { compile_error!("Binary can only be derived on structs and enums") }.into(),
    }
    /*
    let expanded = quote! {
        impl Binary for #ty_name {

        }
    };

    TokenStream::from(expanded)
    */
}
