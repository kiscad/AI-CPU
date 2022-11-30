#[cxx::bridge]
mod ffi {
    // shared structs, whose fields will be visible to both langs
    struct BlobMetadata {
        size: usize,
        tags: Vec<String>,
    }

    extern "Rust" {
        // opaque types, which both langs can pass around
        // bu only Rust can see the fields.
        type MultiBuf;

        // Functions implemented in Rust.
        fn next_chunk(buf: &mut MultiBuf) -> &[u8];
    }

    unsafe extern "C++" {
        // headers with the matching C++ declarations for the
        // enclosing extern "C++" block. Our code generators don't
        // read it but it gets `#include'd` and used in static assertions
        // ensure our picture of the FFI boundary is accurate.
        include!("demo/include/blobstore.h");

        // opaque types which both langs can pass around
        // but only C++ can see the fields.
        type BlobstoreClient;

        // FUnctions implemented in C++.
        fn new_blobstore_client() -> UniquePtr<BlobstoreClient>;
        fn put(&self, parts: &mut MultiBuf) -> u64;
        fn tag(&self, blobid: u64, tag: &str);
        fn metadata(&self, blobid: u64) -> BlobMetadata;
    }
}

struct MultiBuf;

fn next_chunk(buf: &mut MultiBuf) -> &[u8] {
    &[0, 1, 2]
}
