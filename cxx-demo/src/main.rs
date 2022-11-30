use cxx::UniquePtr;

#[cxx::bridge]
mod ffi {

    struct BlobMetadata {
        size: usize,
        tags: Vec<String>,
    }

    unsafe extern "C++" {
        include!("cxx-demo/include/blobstore.h");

        type BlobstoreClient;

        fn new_blobstore_client() -> UniquePtr<BlobstoreClient>;
        fn put(&self, parts: &mut MultiBuf) -> u64;
        fn tag(&self, blobid: u64, tag: &str);
        fn metadata(&self, blobid: u64) -> BlobMetadata;
    }

    extern "Rust" {
        type MultiBuf;

        fn next_chunk(buf: &mut MultiBuf) -> &[u8];
    }
}

pub struct MultiBuf {
    chunks: Vec<Vec<u8>>,
    pos: usize,
}

pub fn next_chunk(buf: &mut MultiBuf) -> &[u8] {
    let next = buf.chunks.get(buf.pos);
    buf.pos += 1;
    next.map_or(&[], Vec::as_slice)
}

fn main() {
    let client = ffi::new_blobstore_client();

    // upload a blob
    let chunks = vec![b"fearless".to_vec(), b"concurrency".to_vec()];
    let mut buf = MultiBuf { chunks, pos: 0 };
    let blobid = client.put(&mut buf);
    println!("blobid = {}", blobid);

    // add a tag.
    client.tag(blobid, "rust");

    // read back the tags
    let metadata = client.metadata(blobid);
    println!("tags = {:?}", metadata.tags);
}
