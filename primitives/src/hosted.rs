// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

#![allow(unused_imports)]
#![allow(dead_code)]

extern crate alloc;

pub use crate::{ArchiveError, Query, RawQuery, ReturnValue};

use bytecheck::CheckBytes;
use microkelvin::{OffsetLen, StoreRef, StoreSerializer};
use rkyv::validation::validators::DefaultValidator;
use rkyv::{Archive, Deserialize, Serialize};

const BUFFER_SIZE_LIMIT: usize = 1024 * 16;

// declare available host-calls
pub mod external {
    extern "C" {
        pub fn debug(buffer: &u8, len: i32);

        pub fn gas_consumed() -> u64;

        pub fn gas_left() -> u64;

        pub fn sha256(input: &u8, input_len: u32, buffer: &mut u8);
    }
}

pub mod env {
    use super::external;
    /// Write debug string
    pub fn log(debug_string: impl AsRef<str>) {
        let mut buffer = [0u8; 1024];
        let string = debug_string.as_ref();
        buffer[..string.len()].copy_from_slice(string.as_bytes());
        unsafe { external::debug(&buffer[0], string.len() as i32) }
    }

    /// Return the amount of gas consumed until the point when the host call is
    /// executed.
    pub fn gas_consumed() -> u64 {
        unsafe { external::gas_consumed() }
    }

    /// Return the ammunt of gas left until the point when the host call is
    /// executed.
    pub fn gas_left() -> u64 {
        unsafe { external::gas_left() }
    }

    pub fn sha256(bytes: &[u8]) -> [u8; 32] {
        let mut result = [0u8; 32];
        unsafe { external::sha256(&bytes[0], bytes.len() as _, &mut result[0]) };

        result
    }
}
