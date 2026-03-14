#![no_main]

use fory_core::buffer::Reader;
use fory_core::meta::FieldInfo;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let mut reader = Reader::new(data);
    let _ = FieldInfo::from_bytes(&mut reader);
});
