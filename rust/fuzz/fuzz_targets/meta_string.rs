#![no_main]

use arbitrary::Arbitrary;
use fory_core::buffer::Reader;
use fory_core::resolver::meta_string_resolver::MetaStringReaderResolver;
use libfuzzer_sys::fuzz_target;

const MAX_WINDOWS: usize = 16;
const MAX_CASES: usize = 16;
const MAX_META_LEN: usize = 32;

#[derive(Arbitrary, Clone, Copy, Debug)]
enum MetaEncoding {
    Utf8,
    LowerSpecial,
    LowerUpperDigitSpecial,
    FirstToLowerSpecial,
    AllToLowerSpecial,
}

impl MetaEncoding {
    #[inline]
    fn byte(self) -> u8 {
        match self {
            MetaEncoding::Utf8 => 0,
            MetaEncoding::LowerSpecial => 1,
            MetaEncoding::LowerUpperDigitSpecial => 2,
            MetaEncoding::FirstToLowerSpecial => 3,
            MetaEncoding::AllToLowerSpecial => 4,
        }
    }
}

#[derive(Arbitrary, Debug)]
struct MetaStringCase {
    len_hint: u8,
    encoding: MetaEncoding,
    payload: Vec<u8>,
    fallback_payload_byte: u8,
    truncate_last_byte: bool,
}

#[derive(Arbitrary, Debug)]
struct MetaStringInput {
    raw: Vec<u8>,
    shifted_starts: Vec<u8>,
    primary_case: MetaStringCase,
    extra_cases: Vec<MetaStringCase>,
}

#[inline]
fn parse_meta_string(data: &[u8]) {
    let mut resolver = MetaStringReaderResolver::default();
    let mut reader = Reader::new(data);
    let _ = resolver.read_meta_string(&mut reader);
}

#[inline]
fn write_var_uint32(out: &mut Vec<u8>, mut value: u32) {
    loop {
        let mut byte = (value & 0x7f) as u8;
        value >>= 7;
        if value != 0 {
            byte |= 0x80;
        }
        out.push(byte);
        if value == 0 {
            return;
        }
    }
}

#[inline]
fn extend_pad(out: &mut Vec<u8>, payload: &[u8], len: usize, pad: u8) {
    let available = payload.len().min(len);
    out.extend_from_slice(&payload[..available]);
    if available < len {
        out.resize(out.len() + (len - available), pad);
    }
}

fn parse_structured_case(case: &MetaStringCase) {
    let len = (case.len_hint as usize).min(MAX_META_LEN);
    let mut candidate = Vec::with_capacity(4 + len);

    write_var_uint32(&mut candidate, (len as u32) << 1);
    if len > 16 {
        candidate.extend_from_slice(&0i64.to_le_bytes());
    } else if len != 0 {
        candidate.push(case.encoding.byte());
    }

    if len != 0 {
        extend_pad(
            &mut candidate,
            &case.payload,
            len,
            case.fallback_payload_byte,
        );
    }

    if case.truncate_last_byte && !candidate.is_empty() {
        candidate.pop();
    }

    parse_meta_string(&candidate);
}

fn parse_raw_and_shifted(raw: &[u8], shifted_starts: &[u8]) {
    parse_meta_string(raw);
    if raw.is_empty() {
        return;
    }

    for start in shifted_starts.iter().take(MAX_WINDOWS) {
        parse_meta_string(&raw[(*start as usize) % raw.len()..]);
    }
}

fuzz_target!(|input: MetaStringInput| {
    parse_raw_and_shifted(&input.raw, &input.shifted_starts);

    for case in std::iter::once(&input.primary_case).chain(input.extra_cases.iter().take(MAX_CASES))
    {
        parse_structured_case(case);
    }
});
