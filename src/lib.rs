#![no_std]
//! Blanket traits providing methods for turning anything into an `Option` or a `Result`.
//!
//! Inspired by [`tap::Pipe`](https://docs.rs/tap/1.0.1/tap/pipe/trait.Pipe.html), which provides
//! a `pipe` method that allows the user to call any bare function in the middle of a method chain.
//!
//! The most useful of these traits' methods is probably [`IntoResult::into_ok`]. Instead of wrapping
//! a long method-chain expression in a call to `Ok`, try tacking an `.into_ok()` at the end. This can
//! be especially pleasing in functions that return a `Result` and make judicious use of method
//! chains and the `?` operator.
//! ```
//! use tailsome::IntoResult;
//!
//! build_pipeline().unwrap();
//!
//! fn build_pipeline() -> Result<Example, Error> {
//!     Builder::new()
//!         .set_option(42)
//!         .try_set_option("some flag")?
//!         .set_option("another option")
//!         .build()
//!         .into_ok()
//! }
//!
//! struct Example;
//!
//! struct Builder;
//! impl Builder {
//!     fn new() -> Self { Self }
//!     fn set_option<T>(self, t: T) -> Self { self }
//!     fn try_set_option<T>(self, t: T) -> Result<Self, Error> { Ok(self) }
//!     fn build(self) -> Example { Example }
//! }
//!
//! #[derive(Debug)]
//! struct Error;
//! ```

/// Provides `into_some`. Implemented for all `Sized` types.
pub trait IntoOption: Sized {
    /// Consumes `self` and returns it wrapped in `Some`.
    fn into_some(self) -> Option<Self>;
}

impl<T: Sized> IntoOption for T {
    fn into_some(self) -> Option<Self> {
        Some(self)
    }
}

/// Provides `into_ok` and `into_err`. Implemented for all `Sized` types.
pub trait IntoResult: Sized {
    /// Consumes `self` and returns it wrapped in `Ok`.
    fn into_ok<E>(self) -> Result<Self, E>;
    /// Consumes `self` and returns it wrapped in `Err`.
    fn into_err<O>(self) -> Result<O, Self>;
}

impl<T: Sized> IntoResult for T {
    fn into_ok<E>(self) -> Result<Self, E> {
        Ok(self)
    }

    fn into_err<O>(self) -> Result<O, Self> {
        Err(self)
    }
}
