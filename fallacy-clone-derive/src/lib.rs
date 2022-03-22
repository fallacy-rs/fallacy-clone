//! Derive macro crate for fallacy-clone.

use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{Data, Fields};

#[proc_macro_derive(TryClone)]
pub fn derive_try_clone(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = TokenStream::from(input);
    let ast = syn::parse2(input).unwrap();
    let output = impl_try_clone(ast);
    output.into()
}

fn impl_try_clone(ast: syn::DeriveInput) -> TokenStream {
    let name = ast.ident;
    let (impl_generics, type_generics, where_clause) = &ast.generics.split_for_impl();

    match &ast.data {
        Data::Struct(data_struct) => {
            let all_fields = match &data_struct.fields {
                Fields::Named(fields) => {
                    let fields = fields.named.iter().map(|field| &field.ident);
                    quote! {
                        Self {#( #fields: ::fallacy_clone::TryClone::try_clone(&self.#fields)?, )*}
                    }
                }
                Fields::Unnamed(fields) => {
                    let fields = (0..fields.unnamed.len()).map(syn::Index::from);
                    quote! {
                        Self(#(::fallacy_clone::TryClone::try_clone(&self.#fields)?,)*)
                    }
                }
                Fields::Unit => quote!(Self),
            };

            quote! {
                impl #impl_generics ::fallacy_clone::TryClone for #name #type_generics #where_clause {
                    #[inline]
                    fn try_clone(&self) -> ::core::result::Result<Self, ::fallacy_clone::AllocError> {
                        Ok(#all_fields)
                    }
                }
            }
        }
        Data::Enum(data_enum) => {
            let all_variants = data_enum.variants.iter().map(|var| match &var.fields {
                Fields::Named(fields) => {
                    let fields = fields.named.iter().map(|field| &field.ident);
                    let fields2 = fields.clone();
                    let fields3 = fields.clone();
                    let variant = &var.ident;
                    quote! {
                        #name::#variant{#(#fields,)*} => #name::#variant {
                            #(#fields2: ::fallacy_clone::TryClone::try_clone(#fields3)?,)*
                        },
                    }
                }
                Fields::Unnamed(fields) => {
                    let fields = (0..fields.unnamed.len()).map(|i| {
                        let mut ident = [0];
                        syn::Ident::new(((b'a' + i as u8) as char).encode_utf8(&mut ident), var.span())
                    });
                    let fields2 = fields.clone();
                    let variant = &var.ident;
                    quote! {
                        #name::#variant(#(#fields,)*) => #name::#variant(
                            #(::fallacy_clone::TryClone::try_clone(#fields2)?,)*
                        ),
                    }
                }
                Fields::Unit => {
                    let variant = &var.ident;
                    quote! {
                        #name::#variant => #name::#variant,
                    }
                }
            });

            quote! {
                impl #impl_generics ::fallacy_clone::TryClone for #name #type_generics #where_clause {
                    #[inline]
                    fn try_clone(&self) -> ::core::result::Result<Self, ::fallacy_clone::AllocError> {
                        Ok(
                            match self {
                                #(#all_variants)*
                            }
                        )
                    }
                }
            }
        }
        Data::Union(_) => panic!("cannot derived TryClone for union"),
    }
}
