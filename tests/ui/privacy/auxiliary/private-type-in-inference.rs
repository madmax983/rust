// Auxiliary crate for testing that type inference does not pick private types.
// This reproduces the pattern from issue #151115 where a macro creates
// crate-local marker types to restrict trait implementations.

#![crate_type = "lib"]

// Private module containing a crate-local marker type.
// This is the EXACT pattern from the original issue.
mod private {
    // This struct is `pub` (has Visibility::Public) but is in a private
    // module, so it can only be accessed via re-export.
    pub struct CurrentCrate;
}

// Re-export through a doc(hidden) path. This makes the type technically
// accessible but not part of the public API. Inference should NOT pick
// types that are only accessible through hidden paths.
#[doc(hidden)]
pub use private::CurrentCrate as PrivateMarker;

// A trait that uses the marker type to restrict implementations
pub trait LocalOnly<CrateMarker> {
    fn check();
}

// Implement the trait for various types with our private marker
impl LocalOnly<private::CurrentCrate> for u8 {
    fn check() {}
}

impl LocalOnly<private::CurrentCrate> for u32 {
    fn check() {}
}

impl LocalOnly<private::CurrentCrate> for PubType {
    fn check() {}
}

// Public function that requires the trait bound.
// When called with `_` for the Marker parameter, inference should NOT
// be able to pick `private::CurrentCrate` from outside this crate.
pub fn assert_local<T, Marker>()
where
    T: LocalOnly<Marker>,
{
}

// Public type that can be used in tests
pub struct PubType;
