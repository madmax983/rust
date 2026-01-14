// Test that explicit use of doc(hidden) types still works.
// Issue: https://github.com/rust-lang/rust/issues/151115
//
// Even though PrivateMarker is #[doc(hidden)], users who explicitly name it
// should be able to use it. The fix only prevents INFERENCE from picking
// hidden types, not explicit use.

//@ aux-build:private-type-in-inference.rs
//@ check-pass

extern crate private_type_in_inference as dep;

fn test_explicit_hidden_type() {
    // Explicitly naming dep::PrivateMarker (even though it's doc(hidden))
    // should work. The doc(hidden) attribute is about documentation, not
    // preventing use. Only inference should be blocked.
    dep::assert_local::<u8, dep::PrivateMarker>();
}

fn main() {
    test_explicit_hidden_type();
}
