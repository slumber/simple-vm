// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

use crate::gas;
use thiserror::Error;
use wasmer_vm::TrapCode;

#[derive(Error, Debug)]
/// The errors that can happen while executing the VM
pub enum VMError {
    /// The Contract Panicked
    #[error("Panicked with message: {0}")]
    ExecutionPanic(String),
    /// Instrumentation Error
    #[error(transparent)]
    InstrumentationError(#[from] InstrumentationError),
    /// Invalid UTF-8
    #[error("Invalid UTF-8")]
    InvalidUtf8,
    /// Error from reading invalid data
    #[error("Invalid data")]
    InvalidData,
    /// Contract execution ran out of gas
    #[error("Contract execution ran out of gas")]
    OutOfGas,
    /// Invalid WASM module
    #[error("Invalid WASM module")]
    InvalidWASMModule,
    /// WASMER export error
    #[error(transparent)]
    WasmerExportError(#[from] wasmer::ExportError),
    /// WASMER runtime error
    #[error(transparent)]
    WasmerRuntimeError(wasmer::RuntimeError),
    /// WASMER  compile error
    #[error(transparent)]
    WasmerCompileError(#[from] wasmer::CompileError),
    /// WASMER instantiation error
    #[error(transparent)]
    WasmerInstantiationError(#[from] wasmer::InstantiationError),
    /// WASMER trap
    #[error("WASMER trap")]
    WasmerTrap(TrapCode),
}

#[derive(Error, Debug)]
pub enum InstrumentationError {
    #[error("gas metering injection")]
    GasMeteringInjection,
    #[error("stack height injection")]
    StackHeightInjection,
    #[error("multiple tables")]
    MultipleTables,
    #[error("max table size")]
    MaxTableSize,
    #[error("invalid bytecode")]
    InvalidByteCode,
    #[error("invalid instruction type")]
    InvalidInstructionType,
}

impl From<gas::GasError> for VMError {
    fn from(_: gas::GasError) -> Self {
        // Currently the only gas error is `GasLimitExceeded`
        VMError::OutOfGas
    }
}

impl From<wasmer::RuntimeError> for VMError {
    fn from(e: wasmer::RuntimeError) -> Self {
        // NOTE: Do not clone before downcasting!
        // `RuntimeError::downcast` calls `Arc::try_unwrap` which will fail to
        // downcast if there is more than one reference to the `Arc`.
        let e = match e.downcast::<VMError>() {
            Ok(vm_error) => return vm_error,
            Err(err) => err,
        };

        match e.clone().to_trap() {
            Some(trap_code) => VMError::WasmerTrap(trap_code),
            None => VMError::WasmerRuntimeError(e),
        }
    }
}
