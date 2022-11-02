//! An example of constructing a strongly-typed wrapper around
//! a string with small-string optimization.
//!
//! The types in this module do not perform any validation or normalization
//! of their values, so every valid UTF-8 string is potentially valid for
//! these types.

use std::{borrow::Cow, error, fmt};

use aliri_braid::braid;
use smartstring::alias::String;

/// An example of a wrapper around a [`smartstring::SmartString`] with
/// small-string optimization
///
/// This type ends in _Buf_, so the borrowed form of this type
/// will be named [`SmartUsername`].
///
/// Because the no type is explicitly named here, the inner field will
/// implicitly use the `String` type in the namespace where it is defined.
#[braid(serde, ref_doc = "A borrowed reference to a string slice wrapper")]
pub struct SmartUsernameBuf;

/// An example of a wrapper with small-string optimization
///
/// This type wraps the around a [`compact_str::CompactString`], but that
/// implementation detail won't be exposed through the type API due to
/// the use of the `no_expose` braid parameter.
#[braid(serde, no_expose)]
pub struct CompactData(compact_str::CompactString);

/// A non-empty [`String`] normalized to lowercase with small-string optimization
///
/// This type maintains an invariant that ensures that a
/// value of this type cannot be constructed that contains
/// invalid data. Data that _can_ be normalized to a valid
/// instance of this type will be.
///
/// Because this type does normalization, the type explicitly
/// does _not_ implement [`Borrow<str>`][::std::borrow::Borrow],
/// as doing so would could violate the contract of that trait,
/// potentially resulting in lost data. If a user of
/// the crate would like to override this, then they can
/// explicitly implement the trait.
#[braid(
    serde,
    no_expose,
    normalizer,
    ref_doc = "A borrowed reference to a non-empty, lowercase string"
)]
pub struct LowerCompactString(compact_str::CompactString);

impl aliri_braid::Validator for LowerCompactString {
    type Error = InvalidString;

    fn validate(raw: &str) -> Result<(), Self::Error> {
        if raw.is_empty() {
            Err(InvalidString::EmptyString)
        } else if raw.chars().any(char::is_uppercase) {
            Err(InvalidString::InvalidCharacter)
        } else {
            Ok(())
        }
    }
}

impl aliri_braid::Normalizer for LowerCompactString {
    fn normalize(s: &str) -> Result<Cow<str>, Self::Error> {
        if s.is_empty() {
            Err(InvalidString::EmptyString)
        } else if s.contains(char::is_uppercase) {
            Ok(Cow::Owned(s.to_lowercase()))
        } else {
            Ok(Cow::Borrowed(s))
        }
    }
}

/// An error indicating that the provided value was invalid
#[derive(Debug)]
pub enum InvalidString {
    EmptyString,
    InvalidCharacter,
}

impl fmt::Display for InvalidString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::EmptyString => f.write_str("string cannot be empty"),
            Self::InvalidCharacter => f.write_str("string contains invalid uppercase character"),
        }
    }
}

impl error::Error for InvalidString {}
aliri_braid::from_infallible!(InvalidString);
