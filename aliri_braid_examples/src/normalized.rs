//! An example of constructing a strongly-typed wrapper around
//! a normalized string value.
//!
//! The types in this module perform validation and normalization prior
//! to allowing the type to be instantiated. If the value is already
//! normalized, then the value is used without modification. Otherwise,
//! an attempt is made to normalize the value. If the value cannot be
//! normalized, then an error will be returned rather than allowing the
//! invalid value to be constructed.
//!
//! Refer to the [`Normalizer`][aliri_braid::Normalizer] implementation
//! for a given type for additional information on what is considered
//! a valid or normalizable value for the type.

use aliri_braid::braid;
use std::borrow::Cow;
use std::{error, fmt};

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

/// A non-empty [`String`] normalized to lowercase
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
///
/// This type includes an explicit parameter indicating that
/// the borrowed form of this type should be named [`LowerStr`].
#[braid(
    serde,
    normalizer,
    ref = "LowerStr",
    ref_doc = "A borrowed reference to a non-empty, lowercase string"
)]
pub struct LowerString;

impl aliri_braid::Validator for LowerString {
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

impl aliri_braid::Normalizer for LowerString {
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
