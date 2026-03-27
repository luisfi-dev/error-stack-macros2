//! [![crates.io latest version](https://img.shields.io/crates/v/error-stack-macros2?label=version&logo=rust)](https://crates.io/crates/error-stack-macros2)
//! [![crates.io downloads](https://img.shields.io/crates/d/error-stack-macros2)](https://crates.io/crates/error-stack-macros2)
//! [![Tests status](https://img.shields.io/github/actions/workflow/status/luisfi-dev/error-stack-macros2/test.yml?branch=master&label=tests)](https://github.com/luisfi-dev/error-stack-macros2/actions/workflows/test.yml)
//! [![Contributor Covenant Code of Conduct](https://img.shields.io/badge/Contributor%20Covenant-3.0-5e0d73?logo=contributorcovenant)](https://github.com/luisfi-dev/error-stack-macros2/blob/master/.github/CODE_OF_CONDUCT.md)
//!
//! Community-made procedural macros for [`error-stack`].
//!
//! # Example
//!
//! Here is the same example shown in the [`error-stack`] `README`, modified to
//! use the macros provided by this crate:
//!
//! ```rust
//! use error_stack::{Report, ResultExt};
//! use error_stack_macros2::Error;
//!
//! #[derive(Debug, Error)]
//! #[display("invalid experiment description")]
//! struct ParseExperimentError;
//!
//! fn parse_experiment(
//!     description: &str
//! ) -> Result<(u64, u64), Report<ParseExperimentError>> {
//!     let value = description
//!         .parse::<u64>()
//!         .attach_with(|| {
//!             format!("{description:?} could not be parsed as experiment")
//!         })
//!         .change_context(ParseExperimentError)?;
//!
//!     Ok((value, 2 * value))
//! }
//!
//! #[derive(Debug, Error)]
//! #[display("experiment error: could not run experiment")]
//! struct ExperimentError;
//!
//! fn start_experiments(
//!     experiment_ids: &[usize],
//!     experiment_descriptions: &[&str],
//! ) -> Result<Vec<u64>, Report<ExperimentError>> {
//!     let experiments = experiment_ids
//!         .iter()
//!         .map(|exp_id| {
//!             let description = experiment_descriptions
//!                 .get(*exp_id)
//!                 .ok_or_else(|| {
//!                     Report::new(ExperimentError)
//!                         .attach(format!(
//!                             "experiment {exp_id} has no valid description")
//!                         )
//!                 })?;
//!
//!             let experiment = parse_experiment(description)
//!                 .attach(format!("experiment {exp_id} could not be parsed"))
//!                 .change_context(ExperimentError)?;
//!
//!             Ok(move || experiment.0 * experiment.1)
//!         })
//!         .collect::<Result<Vec<_>, Report<ExperimentError>>>()
//!         .attach("unable to set up experiments")?;
//!
//!     Ok(experiments.iter().map(|experiment| experiment()).collect())
//! }
//!
//! let experiment_ids = &[0, 2];
//! let experiment_descriptions = &["10", "20", "3o"];
//! let err = start_experiments(experiment_ids, experiment_descriptions)
//!     .unwrap_err();
//!
//! assert_eq!(err.to_string(), "experiment error: could not run experiment");
//! ```
//!
//! # Support
//!
//! Need help using `error-stack-macros2`? Don't hesitate to reach out on
//! [GitHub Discussions](https://github.com/luisfi-dev/error-stack-macros2/discussions/categories/q-a)!
//!
//! # Links
//!
//! -   [Documentation]
//! -   [GitHub](https://github.com/luisfi-dev/error-stack-macros2)
//! -   [crates.io](https://crates.io/crates/error-stack-macros2)
//!
//! # Contributing
//!
//! Before creating an issue, please consider the following:
//!
//! -   Refer to the [documentation] to
//!     make sure the error is actually a bug and not a mistake of your own.
//! -   Make sure the issue hasn't already been reported or suggested.
//! -   Please report any security vulnerabilities privately through
//!     [Security Advisories](https://github.com/luisfi-dev/error-stack-macros2/security/advisories/new).
//! -   After following these steps, you can file an issue using one of our
//!     [templates](https://github.com/luisfi-dev/error-stack-macros2/issues/new/choose).
//!     Please make sure to follow our
//!     [Code of Conduct](https://github.com/luisfi-dev/error-stack-macros2/blob/master/.github/CODE_OF_CONDUCT.md).
//! -   If you wish to [submit a pull request](https://github.com/luisfi-dev/error-stack-macros2/compare)
//!     alongside your issue, please follow our
//!     [contribution guidelines](https://github.com/luisfi-dev/error-stack-macros2/blob/master/.github/CONTRIBUTING.md).
//!
//! # Disclaimer
//!
//! This crate is not affiliated with the official [`error-stack`] crate or its
//! maintainers.
//!
//! [`error-stack`]: https://crates.io/crates/error-stack
//! [documentation]: https://docs.rs/error-stack-macros2

#![deny(unstable_features)]

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

mod types;
use types::ErrorStackDeriveInput;

/// Derive macro for the [`Error`] trait that implements the best practices for
/// [`error-stack`].
///
/// # Overview
/// This derive macro allows you to automatically implement the required
/// [`Display`] and [`Error`] traits for custom types that you want to use as
/// context types in [`error-stack`] [`Report`]s without all the boilerplate.
///
/// The macro has a `display` attribute, which specifies a formatting string to
/// print a value of the given type or enum variant.
///
/// # Examples
///
/// ## Unit struct (recommended)
///
/// ```
/// use error_stack_macros2::Error;
///
/// #[derive(Debug, Error)]
/// #[display("invalid card string")]
/// struct ParseCardError;
/// ```
///
/// ## Enum
///
/// ```
/// use error_stack_macros2::Error;
///
/// #[derive(Debug, Error)]
/// #[display("credit card error")] // optional default
/// enum CreditCardError {
///     #[display("credit card not found")]
///     InvalidInput(String),
///
///     #[display("failed to retrieve credit card")]
///     Other,
/// }
/// ```
///
/// ## Field interpolation (discouraged)
///
/// ```
/// use error_stack_macros2::Error;
///
/// #[derive(Debug, Error)]
/// #[display("invalid card string: {0:?}")]
/// struct ParseCardError(String);
///
/// let err = ParseCardError("1234567".to_string());
/// assert_eq!(err.to_string(), "invalid card string: \"1234567\"");
/// ```
///
/// # This may look familiar...
///
/// This derive macro is heavily inspired by the popular [`thiserror`] crate. In
/// fact, you **can** use the [`thiserror`] crate to derive the same traits for
/// your types. However, [`error-stack`] is very opinionated about how context
/// types should be designed and used, and this derive macro enforces those
/// best practices, whereas [`thiserror`] is more flexible and designed for
/// general use cases.
///
/// Also, due to this macro's more simple and restricted design, it can
/// potentially be more efficient than [`thiserror`] in terms of compile time
/// and generated code size.
///
/// [`Error`]: core::error::Error
/// [`error-stack`]: https://crates.io/crates/error-stack
/// [`Report`]: https://docs.rs/error-stack/latest/error_stack/struct.Report.html
/// [`Display`]: core::fmt::Display
/// [`thiserror`]: https://crates.io/crates/thiserror
// TODO: remove #[inline] when fixed
#[inline]
#[proc_macro_derive(Error, attributes(display))]
pub fn impl_error_stack(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as ErrorStackDeriveInput);
    quote! { #derive_input }.into()
}
