// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

use tracing::trace;

use crate::env::Env;
use crate::VMError;

pub struct Sha256;

impl Sha256 {
    pub fn sha256(env: &Env, input: i32, input_len: i32, output: i32) -> Result<(), VMError> {
        trace!("Executing 'sha256' host function");

        let context = env.get_context();

        let config = context.config();
        context.charge_gas(config.host_costs.sha256)?;

        let input_ofs = input as u64;
        let msg_input_len = input_len as usize;
        let input_memory = context.read_memory(input_ofs, msg_input_len)?;

        let out = sha256::digest(input_memory);

        let _result_ofs = output;

        context.write_memory(out.as_bytes(), _result_ofs as u64)
    }
}
