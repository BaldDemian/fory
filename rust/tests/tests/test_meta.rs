// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use fory_core::meta::{FieldInfo, FieldType, MetaString, TypeMeta};
use fory_core::types::TypeId;
use fory_core::Reader;

#[test]
fn test_meta_hash() {
    let meta = TypeMeta::new(
        42,
        1,
        MetaString::get_empty().clone(),
        MetaString::get_empty().clone(),
        false,
        vec![FieldInfo {
            field_id: 43,
            field_name: "f1".to_string(),
            field_type: FieldType {
                type_id: TypeId::BOOL as u32,
                user_type_id: u32::MAX,
                nullable: true,
                track_ref: false,
                generics: vec![],
            },
        }],
    )
    .unwrap();
    assert_ne!(meta.get_hash(), 0);
}

#[test]
fn invalid_field_name_bytes_should_return_error_not_panic() {
    // Field header:
    // - encoding bits: 01 (ALL_TO_LOWER_SPECIAL)
    // - size bits: 0 => name length = 1
    // - nullable/track_ref: false
    let bytes = [0x40, TypeId::UNKNOWN as u8, 0x78];
    // 0x78 produces LOWER_SPECIAL char value 30 (>29), which is invalid.
    let mut reader = Reader::new(&bytes);
    let result = FieldInfo::from_bytes(&mut reader);
    assert!(result.is_err());
}

#[test]
fn field_id_extension_overflow_should_return_error_not_panic() {
    // Field header:
    // - encoding bits: 11 (FIELD_ID mode)
    // - size bits: 1111 => requires extended varuint32 field id part
    // - nullable/track_ref: false
    let bytes = [0xFC, 0xFF, 0xFF, 0x01, TypeId::UNKNOWN as u8];
    // extended part = 32767, computed field id = 15 + 32767 => i16 overflow
    let mut reader = Reader::new(&bytes);
    let result = FieldInfo::from_bytes(&mut reader);
    assert!(result.is_err());
}
