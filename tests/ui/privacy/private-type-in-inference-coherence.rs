// Test that coherence still correctly detects overlapping impls
// even when one uses a private marker type.
// Issue: https://github.com/rust-lang/rust/issues/151115
//
// This ensures our fix for inference doesn't break coherence checking.

//@ aux-build:private-type-in-inference.rs
//@ check-pass

extern crate private_type_in_inference as dep;

// Our local marker type
pub struct LocalMarker;

// This impl should NOT conflict with dep's impl that uses PrivateMarker,
// because PrivateMarker and LocalMarker are different types.
impl dep::LocalOnly<LocalMarker> for u8 {
    fn check() {}
}

// This would be a coherence violation if we tried to impl with the same
// marker type, but we can't even name PrivateMarker from here.

fn main() {}
