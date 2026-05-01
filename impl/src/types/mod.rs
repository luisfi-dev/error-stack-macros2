use proc_macro2::TokenStream as TokenStream2;
use quote::{ToTokens, quote};
use syn::{
    Attribute, DeriveInput, Generics, Ident,
    parse::{Parse, ParseStream},
};

mod fmt;
use fmt::TypeData;

mod util;
use util::ReducedGenerics;

pub(crate) struct ErrorStackDeriveInput {
    other_attrs: Vec<Attribute>,
    ident: Ident,
    generics: Generics,
    display_data: TypeData,
}

impl Parse for ErrorStackDeriveInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let derive_input: DeriveInput = input.parse()?;

        drop(derive_input.vis);

        let mut attrs = derive_input.attrs;

        let display_data = TypeData::new(
            derive_input.data,
            &mut attrs,
            derive_input.ident.span(),
        )?;

        let ident = derive_input.ident;

        let mut generics = derive_input.generics;
        generics
            .params
            .iter_mut()
            .for_each(util::remove_generic_default);

        Ok(Self {
            other_attrs: attrs,
            ident,
            generics,
            display_data,
        })
    }
}

impl ToTokens for ErrorStackDeriveInput {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let Self {
            ref other_attrs,
            ref ident,
            ref generics,
            ref display_data,
        } = *self;

        let where_clause = &generics.where_clause;

        let mut error_trait_generics = generics.clone();
        error_trait_generics
            .params
            .iter_mut()
            .for_each(util::add_debug_trait_bound);

        let type_generics: ReducedGenerics = generics
            .params
            .iter()
            .cloned()
            .map(util::generic_reduced_to_ident)
            .collect();

        tokens.extend(quote! {
            #[allow(single_use_lifetimes)]
            #(#other_attrs)*
            impl #generics ::core::fmt::Display for #ident #type_generics
            #where_clause
            {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.write_str(#display_data)
                }
            }

            #[allow(single_use_lifetimes)]
            #(#other_attrs)*
            impl #error_trait_generics ::core::error::Error for #ident #type_generics
            #where_clause
            {
            }
        });
    }
}

#[cfg(test)]
#[expect(
    clippy::expect_used,
    reason = "this is a test module with calls to `.expect()`"
)]
mod tests {
    use quote::quote;

    use crate::ErrorStackDeriveInput;

    #[test]
    fn input_works_with_other_attrs() {
        let input: ErrorStackDeriveInput = syn::parse2(quote! {
            #[test_attribute]
            #[display("custom type")]
            #[test_attribute_2]
            struct CustomType;
        })
        .expect("malformed test stream");

        let output = quote! { #input };
        assert_eq!(
            output.to_string(),
            "# [allow (single_use_lifetimes)] # [test_attribute] # [test_attribute_2] impl :: core :: fmt :: Display for CustomType { fn fmt (& self , f : & mut :: core :: fmt :: Formatter < '_ >) -> :: core :: fmt :: Result { :: core :: write ! (f , \"custom type\" ,) } } # [allow (single_use_lifetimes)] # [test_attribute] # [test_attribute_2] impl :: core :: error :: Error for CustomType { }"
        );
    }

    #[test]
    fn generics_work_with_attrs() {
        let derive_input: ErrorStackDeriveInput = syn::parse2(quote! {
            #[display("custom type")]
            struct CustomType<#[cfg(true)] T> {
                _data: PhantomData<T>
            }
        })
        .expect("malformed test stream");

        let output = quote! { #derive_input };
        assert_eq!(
            output.to_string(),
            "# [allow (single_use_lifetimes)] impl < # [cfg (true)] T > :: core :: fmt :: Display for CustomType < T > { fn fmt (& self , f : & mut :: core :: fmt :: Formatter < '_ >) -> :: core :: fmt :: Result { :: core :: write ! (f , \"custom type\" ,) } } # [allow (single_use_lifetimes)] impl < # [cfg (true)] T : :: core :: fmt :: Debug > :: core :: error :: Error for CustomType < T > { }"
        );
    }

    #[test]
    fn output_impl_has_attr_allow_single_use_lifetimes() {
        let input: ErrorStackDeriveInput = syn::parse2(quote! {
            #[display("custom type")]
            struct CustomType;
        })
        .expect("malformed test stream");

        let output = quote! { #input };
        assert_eq!(
            output.to_string(),
            "# [allow (single_use_lifetimes)] impl :: core :: fmt :: Display for CustomType { fn fmt (& self , f : & mut :: core :: fmt :: Formatter < '_ >) -> :: core :: fmt :: Result { :: core :: write ! (f , \"custom type\" ,) } } # [allow (single_use_lifetimes)] impl :: core :: error :: Error for CustomType { }"
        );
    }
}
