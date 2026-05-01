use std::convert::Infallible;

use proc_macro2::Span;
use syn::{
    Attribute, Meta, Variant, parse::Parse, punctuated::Punctuated,
    spanned::Spanned as _, token::Comma,
};

use super::{super::util, ValidVariantState, VariantData, VariantState};

pub(crate) fn get_format_input<T>(display_attr: Attribute) -> syn::Result<T>
where
    T: Parse,
{
    let attr_span = display_attr.span();

    if let Meta::List(meta) = display_attr.meta {
        let meta_span = meta.span();
        drop(meta.path);

        let parse_res = syn::parse2::<T>(meta.tokens);

        match parse_res {
            Ok(input) => return Ok(input),
            Err(err) => {
                return Err(
                    if err.to_string()
                        == "unexpected end of input, expected string literal"
                    {
                        drop(err);

                        syn::Error::new(
                            meta_span,
                            "unexpected empty `display` attribute, expected string literal",
                        )
                    } else {
                        err
                    },
                );
            }
        }
    }

    drop(display_attr);

    Err(syn::Error::new(
        attr_span,
        "expected `display` to be a list attribute: `#[display(\"template...\")]`",
    ))
}

pub(crate) fn collect_valid_variant_states(
    variants: Punctuated<Variant, Comma>,
) -> Result<Vec<ValidVariantState>, syn::Error> {
    let mut variant_states_iter = variants.into_iter().map(|variant| {
        use super::FieldsType as FT;
        use VariantState as VS;
        use syn::Fields as F;

        let variant_span = variant.span();
        drop(variant.discriminant);

        let mut attrs = variant.attrs;
        let display_attr = util::take_display_attr(&mut attrs);

        match display_attr {
            None => {
                drop(variant.fields);
                drop(variant.ident);
                drop(attrs);
                drop(display_attr);

                VS::None(variant_span)
            }

            Some(attr) => match get_format_input(attr) {
                Ok(input) => VS::Valid(VariantData {
                    other_attrs: attrs,
                    ident: variant.ident,
                    fields_type: match variant.fields {
                        F::Named(_) => FT::Named,
                        F::Unnamed(_) => FT::Unnamed,
                        F::Unit => FT::Unit,
                    },
                    display_input: input,
                }),
                Err(err) => VS::Invalid(err),
            },
        }
    });

    let mut vec = Vec::new();

    while let Some(state) = variant_states_iter.next() {
        use VariantState as VS;
        match state {
            VS::None(span) => vec.push(VS::None(span)),
            VS::Valid(data) => vec.push(VS::Valid(data)),
            VS::Invalid(mut err) => {
                while let Some(VS::Invalid(err2)) = variant_states_iter.next() {
                    err.combine(err2);
                }

                drop(variant_states_iter);
                return Err(err);
            }
        }
    }

    drop(variant_states_iter);

    Ok(vec)
}

pub(crate) fn separate_existing_variant_states(
    states_iter: Vec<VariantState<Infallible>>,
) -> (Vec<VariantData>, Vec<Span>) {
    let mut valid_variants = Vec::new();
    let mut none_spans = Vec::new();

    for state in states_iter {
        use VariantState as VS;
        match state {
            VS::Valid(data) => valid_variants.push(data),
            VS::None(span) => none_spans.push(span),
        }
    }

    (valid_variants, none_spans)
}
