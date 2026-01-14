// Test that type inference does not pick inaccessible foreign types.
// Issue: https://github.com/rust-lang/rust/issues/151115
//
// When only a foreign impl exists with a private marker type, inference
// should fail because there's no accessible impl.

//@ aux-build:private-type-in-inference.rs

extern crate private_type_in_inference as dep;

fn test_no_local_impl() {
    // u32 only has an impl in dep with private marker type (PrivateMarker).
    // Since we can't access `dep::PrivateMarker`, inference should not be
    // able to find a valid impl.
    //
    // Before fix: Compiled! Inference incorrectly picked the private type.
    // After fix: Error - the trait bound is not satisfied
    dep::assert_local::<u32, _>();
    //~^ ERROR the trait bound `u32: LocalOnly<_>` is not satisfied
}

fn main() {
    test_no_local_impl();
}
