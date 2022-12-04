// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

use crate::env::Env;
use crate::ops::*;

use wasmer::{Exports, Function, Store};

pub struct HostImportsResolver;

impl HostImportsResolver {
    pub fn insert_into_namespace(
        namespace: &mut Exports,
        store: &Store,
        env: Env,
        names: &[String],
    ) {
        for name in names {
            match name.as_str() {
                "debug" => namespace.insert(
                    "debug",
                    Function::new_native_with_env(store, env.clone(), debug::Debug::debug),
                ),
                "gas_consumed" => namespace.insert(
                    "gas_consumed",
                    Function::new_native_with_env(
                        store,
                        env.clone(),
                        gas::GasConsumed::gas_consumed,
                    ),
                ),
                "gas_left" => namespace.insert(
                    "gas_left",
                    Function::new_native_with_env(store, env.clone(), gas::GasLeft::gas_left),
                ),
                "sha256" => namespace.insert(
                    "sha256",
                    Function::new_native_with_env(store, env.clone(), sha256::Sha256::sha256),
                ),
                _ => {
                    debug_assert!(false, "unknown wasm module import {}", name)
                }
            }
        }
    }
}
