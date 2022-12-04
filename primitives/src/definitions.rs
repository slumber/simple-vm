// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

extern crate alloc;

use core::fmt::Debug;

use alloc::boxed::Box;
use alloc::vec::Vec;

use bytecheck::CheckBytes;
use microkelvin::{OffsetLen, StoreRef, StoreSerializer};
use rkyv::{
    archived_root, check_archived_root,
    ser::Serializer,
    validation::{
        validators::{DefaultValidator, DefaultValidatorError},
        CheckArchiveError,
    },
    Archive, Serialize,
};

pub type StoreContext = StoreRef<OffsetLen>;

pub trait Execute<Q>
where
    Q: Query,
{
    fn execute(&self, q: Q, store: StoreContext) -> Q::Return;
}

pub trait Query: Archive {
    const NAME: &'static str;

    type Return;
}

// TODO, use borrowed bytes here?
#[derive(Debug, Default)]
pub struct ReturnValue {
    data: Box<[u8]>,
    state: Box<[u8]>,
}

impl ReturnValue {
    pub fn new(result: impl AsRef<[u8]>) -> Self {
        let result = Box::from(result.as_ref());
        ReturnValue {
            data: result,
            state: Box::from([].as_ref()),
        }
    }

    pub fn with_state(result: impl AsRef<[u8]>, state: impl AsRef<[u8]>) -> Self {
        let result = Box::from(result.as_ref());
        let state = Box::from(state.as_ref());
        ReturnValue {
            data: result,
            state,
        }
    }

    pub fn cast<'a, T>(
        &'a self,
    ) -> Result<
        &'a T::Archived,
        CheckArchiveError<
            <T::Archived as CheckBytes<DefaultValidator<'a>>>::Error,
            DefaultValidatorError,
        >,
    >
    where
        T: Archive,
        T::Archived: CheckBytes<DefaultValidator<'a>>,
    {
        check_archived_root::<T>(&self.data[..])
    }

    pub fn cast_state<T>(&self) -> &T::Archived
    where
        T: Archive,
    {
        let state: &T::Archived = unsafe { archived_root::<T>(&self.state[..]) };
        state
    }

    pub fn cast_data<T>(&self) -> &T::Archived
    where
        T: Archive,
    {
        let data: &T::Archived = unsafe { archived_root::<T>(&self.data[..]) };
        data
    }

    pub fn data_len(&self) -> usize {
        self.data.len()
    }

    pub fn data(&self) -> &[u8] {
        &self.data[..]
    }

    pub fn state_len(&self) -> usize {
        self.state.len()
    }

    pub fn state(&self) -> &[u8] {
        &self.state[..]
    }

    pub fn encode_lenghts(&self) -> u64 {
        ((self.data_len() as u64 + self.state_len() as u64) << 32) + self.state_len() as u64
    }
}

#[derive(Debug, Default)]
pub struct RawQuery<'a> {
    data: Vec<u8>,
    name: &'a str,
}

impl<'a> RawQuery<'a> {
    pub fn new<Q>(q: Q, store: &StoreRef<OffsetLen>) -> Self
    where
        Q: Query + Serialize<StoreSerializer<OffsetLen>>,
    {
        let mut ser = store.serializer();
        ser.serialize_value(&q).unwrap();
        RawQuery {
            data: ser.spill_bytes(|bytes| Vec::from(bytes)),
            name: Q::NAME,
        }
    }

    pub fn from<D: Into<Vec<u8>>>(data: D, name: &'a str) -> Self {
        Self {
            data: data.into(),
            name,
        }
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn data(&self) -> &[u8] {
        &self.data[..]
    }
}

#[derive(Debug)]
pub enum ArchiveError {
    ArchiveValidationError,
}
