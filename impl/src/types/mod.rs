use std::{borrow::Borrow, ops::Deref};

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

use crate::types::util::ATTRIBUTES_TO_REMOVE;

pub(crate) struct ErrorStackDeriveInput {
    other_attrs: OtherAttributes,
    ident: Ident,
    generics: Generics,
    display_data: TypeData,
}

impl Parse for ErrorStackDeriveInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let derive_input: DeriveInput = input.parse()?;
        drop(derive_input.vis);

        let (other_attrs, display_attr) =
            OtherAttributes::take_display_and_remove_incompatible_from(
                derive_input.attrs,
            );

        let display_data = TypeData::new(
            derive_input.data,
            display_attr,
            derive_input.ident.span(),
        )?;

        let ident = derive_input.ident;

        let mut generics = derive_input.generics;
        generics
            .params
            .iter_mut()
            .for_each(util::remove_generic_default);

        Ok(Self {
            other_attrs,
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

        let other_attrs_iter = other_attrs.iter();
        let other_attrs_iter2 = other_attrs.iter();

        tokens.extend(quote! {
            #[allow(single_use_lifetimes)]
            #(#other_attrs_iter)*
            impl #generics ::core::fmt::Display for #ident #type_generics
            #where_clause
            {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    f.write_str(#display_data)
                }
            }

            #[allow(single_use_lifetimes)]
            #(#other_attrs_iter2)*
            impl #error_trait_generics ::core::error::Error for #ident #type_generics
            #where_clause
            {
            }
        });
    }
}

pub(crate) struct OtherAttributes {
    inner: Vec<Attribute>,
}

impl OtherAttributes {
    fn take_display_and_remove_incompatible_from(
        attrs: Vec<Attribute>,
    ) -> (OtherAttributes, Option<Attribute>) {
        let mut other_attrs_vec = Vec::with_capacity(attrs.capacity());
        let mut display_attr = None::<Attribute>;

        for attr in attrs {
            if attr.path().is_ident("display") {
                display_attr = Some(attr);
            } else if !ATTRIBUTES_TO_REMOVE
                .iter()
                .any(|name| attr.path().is_ident(name))
            {
                other_attrs_vec.push(attr);
            }
        }

        let other_attrs = OtherAttributes {
            inner: other_attrs_vec,
        };

        (other_attrs, display_attr)
    }
}

impl Borrow<[Attribute]> for OtherAttributes {
    fn borrow(&self) -> &[Attribute] {
        self.inner.borrow()
    }
}

impl Deref for OtherAttributes {
    type Target = [Attribute];

    fn deref(&self) -> &Self::Target {
        self.inner.deref()
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
