#![no_main]

use arbitrary::Arbitrary;
use fory_core::buffer::Reader;
use fory_core::meta::FieldInfo;
use libfuzzer_sys::fuzz_target;

const MAX_WINDOWS: usize = 16;
const MAX_CASES: usize = 16;
const MAX_NAME_LEN: usize = 16;

#[derive(Arbitrary, Clone, Copy, Debug)]
enum FieldNameEncoding {
    Utf8,
    AllToLowerSpecial,
    LowerUpperDigitSpecial,
    FieldId, // triggers encoding_bits == 0b11 (FIELD_ID_ENCODING_MARKER branch)
}

impl FieldNameEncoding {
    #[inline]
    fn bits(self) -> u8 {
        match self {
            FieldNameEncoding::Utf8 => 0,
            FieldNameEncoding::AllToLowerSpecial => 1,
            FieldNameEncoding::LowerUpperDigitSpecial => 2,
            FieldNameEncoding::FieldId => 3,
        }
    }
}

#[derive(Arbitrary, Debug)]
struct FieldNameModeCase {
    encoding: FieldNameEncoding,
    nullable: bool,
    track_ref: bool,
    type_id: u8,
    fallback_name_byte: u8,
    name_payload: Vec<u8>,
    truncate_last_byte: bool,
}

#[derive(Arbitrary, Debug)]
struct TypeMetaInput {
    raw: Vec<u8>,
    shifted_starts: Vec<u8>,
    primary_case: FieldNameModeCase,
    extra_cases: Vec<FieldNameModeCase>,
}

#[inline]
fn parse_field_info(data: &[u8]) {
    let mut reader = Reader::new(data);
    let _ = FieldInfo::from_bytes(&mut reader);
}

#[inline]
fn extend_padded(out: &mut Vec<u8>, payload: &[u8], len: usize, pad: u8) {
    let available = payload.len().min(len);
    out.extend_from_slice(&payload[..available]);
    if available < len {
        out.resize(out.len() + (len - available), pad);
    }
}

fn parse_field_name_mode_case(case: &FieldNameModeCase) {
    let name_len = case.name_payload.len().clamp(1, MAX_NAME_LEN);
    let mut header = case.encoding.bits() << 6;
    header |= ((name_len - 1) as u8) << 2;
    if case.nullable {
        header |= 0b10;
    }
    if case.track_ref {
        header |= 0b01;
    }

    let mut candidate = Vec::with_capacity(2 + name_len);
    candidate.push(header);
    candidate.push(case.type_id);
    extend_padded(
        &mut candidate,
        &case.name_payload,
        name_len,
        case.fallback_name_byte,
    );

    if case.truncate_last_byte && candidate.len() > 2 {
        candidate.pop();
    }

    parse_field_info(&candidate);
}

fn parse_raw_and_shifted(raw: &[u8], shifted_starts: &[u8]) {
    parse_field_info(raw);
    if raw.is_empty() {
        return;
    }

    for start in shifted_starts.iter().take(MAX_WINDOWS) {
        parse_field_info(&raw[(*start as usize) % raw.len()..]);
    }
}

fuzz_target!(|input: TypeMetaInput| {
    parse_raw_and_shifted(&input.raw, &input.shifted_starts);

    for case in std::iter::once(&input.primary_case).chain(input.extra_cases.iter().take(MAX_CASES))
    {
        parse_field_name_mode_case(case);
    }
});
