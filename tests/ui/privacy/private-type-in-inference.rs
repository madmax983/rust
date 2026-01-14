// Test that type inference does not pick private/inaccessible types.
// Issue: https://github.com/rust-lang/rust/issues/151115
//
// When both a local impl and a foreign impl (with private marker type) exist,
// inference should only see the local impl (the foreign impl uses a type
// with restricted visibility that cannot be accessed from this crate).

//@ aux-build:private-type-in-inference.rs
//@ check-pass

extern crate private_type_in_inference as dep;

// Local marker type
mod local {
    pub struct LocalMarker;
}

// Implement dep's trait with our local marker type
impl dep::LocalOnly<local::LocalMarker> for u8 {
    fn check() {}
}

impl dep::LocalOnly<local::LocalMarker> for dep::PubType {
    fn check() {}
}

fn test_no_ambiguity_u8() {
    // Before fix: E0283 ambiguity error (sees both local and foreign impls)
    // After fix: Compiles successfully (foreign impl with private type filtered)
    dep::assert_local::<u8, _>();
}

fn test_no_ambiguity_pub_type() {
    // Same pattern with dep::PubType
    dep::assert_local::<dep::PubType, _>();
}

fn main() {
    test_no_ambiguity_u8();
    test_no_ambiguity_pub_type();
}
