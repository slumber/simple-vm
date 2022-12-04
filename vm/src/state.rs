// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

use tracing::{trace, trace_span};

use crate::call_context::CallContext;
use crate::config::{Config, DEFAULT_CONFIG};
use crate::error::VMError;
use crate::gas::GasMeter;

/// WASM stack based virtual machine.
#[derive(Clone)]
pub struct Vm {
    config: &'static Config,
}

impl Vm {
    /// Returns a new empty [`Vm`] with the default configuration.
    pub fn new() -> Self {
        Vm {
            config: &DEFAULT_CONFIG,
        }
    }

    /// Returns a new empty [`Vm`] with the given configuration.
    pub fn with_config(config: &'static Config) -> Self {
        Vm { config }
    }

    /// Returns the configuration of this instance.
    pub fn config(&self) -> &'static Config {
        self.config
    }

    /// Execute wasm with the given entrypoint.
    pub fn execute(
        &self,
        code: &[u8],
        entrypoint: &str,
        gas_meter: &mut GasMeter,
    ) -> Result<(), VMError> {
        let _span = trace_span!(
            "outer query",
            gas_limit = ?gas_meter.limit()
        );

        let mut state = self.clone();

        let mut context = CallContext::new(&mut state);

        let entrypoint = format!("__vm_{}", entrypoint);
        match context.execute(code, &entrypoint, gas_meter) {
            Ok(result) => {
                trace!("query was successful");
                Ok(result)
            }
            Err(e) => {
                trace!("query returned an error: {}", e);
                Err(e)
            }
        }?;

        Ok(())
    }
}

impl Default for Vm {
    fn default() -> Self {
        Self::new()
    }
}
