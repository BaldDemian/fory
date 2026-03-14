#![no_main]

use fory_core::buffer::Reader;
use fory_core::resolver::meta_string_resolver::MetaStringReaderResolver;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let mut resolver = MetaStringReaderResolver::default();
    let mut reader = Reader::new(data);
    let _ = resolver.read_meta_string(&mut reader);
});
