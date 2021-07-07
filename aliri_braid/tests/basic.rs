use aliri_braid::braid;

/// A basic example of a wrapper around a [`String`]
#[braid(
    serde,
    ref_doc = "A borrowed reference to a basic string slice wrapper"
)]
pub struct BasicExampleBuf;

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Borrow;

    #[test]
    fn owned_works() {
        let x = BasicExampleBuf::new("Testing the Buffer");
        assert_eq!(x.as_str(), "Testing the Buffer");
    }

    #[test]
    fn borrowing_implicit() {
        let x: &BasicExample = &BasicExampleBuf::new("Testing the Buffer");
        assert_eq!(x.as_str(), "Testing the Buffer");
    }

    #[test]
    fn ref_works() {
        let x = BasicExample::from_str("Testing the Reference");
        assert_eq!(x.as_str(), "Testing the Reference");
    }

    #[allow(dead_code)]
    struct Bar<'a> {
        foo: std::borrow::Cow<'a, BasicExample>,
    }

    #[test]
    fn owned_as_cow() {
        let owned = BasicExampleBuf::new("Testing the Buffer");
        let _bar = Bar {
            foo: owned.into(),
        };
    }

    #[test]
    fn borrowed_as_cow() {
        let borrowed = BasicExample::from_str("Testing the Reference");
        let _bar = Bar {
            foo: borrowed.into(),
        };
    }

    #[test]
    fn owned_as_ref_borrowed() {
        let owned = BasicExampleBuf::new("Testing the Buffer");
        let _reference: &BasicExample = owned.as_ref();
    }

    #[test]
    fn owned_as_ref_str() {
        let owned = BasicExampleBuf::new("Testing the Buffer");
        let _reference: &str = owned.as_ref();
    }

    #[test]
    fn borrowed_as_ref_str() {
        let owned = BasicExample::from_str("Testing the Buffer");
        let _reference: &str = owned.as_ref();
    }

    #[test]
    fn owned_borrow_borrowed() {
        let owned = BasicExampleBuf::new("Testing the Buffer");
        let _reference: &BasicExample = owned.borrow();
    }

    #[test]
    fn owned_borrow_str() {
        let owned = BasicExampleBuf::new("Testing the Buffer");
        let _reference: &str = owned.borrow();
    }

    #[test]
    fn borrowed_borrow_str() {
        let owned = BasicExample::from_str("Testing the Buffer");
        let _reference: &str = owned.borrow();
    }
}
