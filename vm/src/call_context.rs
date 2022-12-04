// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

#![allow(dead_code)]

use primitives::ReturnValue;

use tracing::{trace, trace_span};
use wasmer::{Exports, ImportObject, Instance, LazyInit, Module, NativeFunc};
use wasmer_middlewares::metering::set_remaining_points;

use crate::compiler::WasmerCompiler;
use crate::env::Env;
use crate::gas::{Gas, GasMeter};
use crate::memory::WasmerMemory;
use crate::resolver::HostImportsResolver;
use crate::state::Vm;
use crate::{Config, VMError};

pub struct StackFrame {
    ret: ReturnValue,
    memory: WasmerMemory,
    gas_meter: GasMeter,
    instance: Instance,
}

impl std::fmt::Debug for StackFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(return: {:?})", self.ret)
    }
}

impl StackFrame {
    fn new(memory: WasmerMemory, gas_meter: GasMeter, instance: Instance) -> StackFrame {
        StackFrame {
            memory,
            ret: Default::default(),
            gas_meter,
            instance,
        }
    }

    fn write_memory(&mut self, source_slice: &[u8], offset: u64) -> Result<(), VMError> {
        self.memory.write(offset, source_slice)
    }

    fn read_memory(&self, offset: u64, length: usize) -> Result<&[u8], VMError> {
        self.memory.read(offset, length)
    }
}

pub struct CallContext<'a> {
    state: &'a mut Vm,
    stack: Vec<StackFrame>,
}

impl<'a> CallContext<'a> {
    pub fn new(state: &'a mut Vm) -> Self {
        CallContext {
            state,
            stack: vec![],
        }
    }

    fn register_namespace(
        namespace_name: &str,
        env: &Env,
        module: &Module,
        import_names: &[String],
        import_object: &mut ImportObject,
    ) {
        let mut namespace = Exports::new();
        HostImportsResolver::insert_into_namespace(
            &mut namespace,
            module.store(),
            env.clone(),
            import_names,
        );
        import_object.register(namespace_name, namespace);
    }

    pub fn execute(
        &mut self,
        bytecode: &[u8],
        entrypoint: &str,
        gas_meter: &'a mut GasMeter,
    ) -> Result<(), VMError> {
        let _span = trace_span!(
            "query",
            gas_limit = ?gas_meter.limit(),
            stack_index = ?self.stack.len()
        );

        let env = Env::new(self);

        let instance: Instance;

        let r = {
            let module = WasmerCompiler::create_module(bytecode, self.state.config())?;

            let import_names: Vec<String> =
                module.imports().map(|i| i.name().to_string()).collect();
            let mut import_object = ImportObject::new();

            CallContext::register_namespace(
                "env",
                &env,
                &module,
                &import_names,
                &mut import_object,
            );

            instance = Instance::new(&module, &import_object)?;
            set_remaining_points(&instance, gas_meter.left());

            let mut memory = WasmerMemory {
                inner: LazyInit::new(),
            };
            memory.init(&instance.exports)?;

            self.stack
                .push(StackFrame::new(memory, gas_meter.clone(), instance.clone()));

            let run_func: NativeFunc<(), ()> = instance.exports.get_native_function(entrypoint)?;

            let mut memory = WasmerMemory::new();
            memory.init(&instance.exports)?;

            let r = run_func.call();

            r
        };

        match self.gas_reconciliation() {
            Ok(gas) => *gas_meter = gas,
            Err(e) => {
                gas_meter.exhaust();
                return Err(e);
            }
        }
        trace!(
            "Finished query with gas limit/spent: {}/{}",
            gas_meter.limit(),
            gas_meter.spent()
        );

        let result = r.map_err(|a| VMError::ExecutionPanic(a.message()))?;
        self.stack.pop();
        Ok(result)
    }

    pub fn gas_meter(&mut self) -> Result<&GasMeter, VMError> {
        let stack = &mut self.top_mut();
        let instance = &stack.instance;
        let gas_meter = &mut stack.gas_meter;

        gas_meter.update(instance, 0)?;

        Ok(&self.top().gas_meter)
    }

    /// Charge gas to the meter in the topmost stack frame.
    pub fn charge_gas(&mut self, gas: Gas) -> Result<(), VMError> {
        let frame = &mut self.top_mut();
        let instance = &frame.instance;
        let gas_meter = &mut frame.gas_meter;

        gas_meter.update(instance, gas)?;

        Ok(())
    }

    pub fn config(&self) -> &'static Config {
        self.state.config()
    }

    pub fn top(&self) -> &StackFrame {
        self.stack.last().expect("Stack should not be empty")
    }

    pub fn top_mut(&mut self) -> &mut StackFrame {
        self.stack.last_mut().expect("Stack should not be empty")
    }

    pub fn read_memory(&self, offset: u64, length: usize) -> Result<&[u8], VMError> {
        self.top().read_memory(offset, length)
    }

    pub fn write_memory(&mut self, source_slice: &[u8], offset: u64) -> Result<(), VMError> {
        self.top_mut().write_memory(source_slice, offset)?;
        Ok(())
    }

    /// Reconcile the gas usage across the stack.
    fn gas_reconciliation(&mut self) -> Result<GasMeter, VMError> {
        // If there is more than one [`StackFrame`] on the stack, then the
        // gas needs to be reconciled.
        if self.stack.len() > 1 {
            let len = self.stack.len() - 2;
            let spent = self.gas_meter()?.spent();
            let parent = &mut self.stack[len];
            let parent_meter = &mut parent.gas_meter;
            let parent_instance = &parent.instance;

            // FIXME: This is a hack to make sure that the gas meter's parent
            // consumes the gas spent from its own inter-contract calls.
            // It doesn't take in account arbitrary `charge` calls to `GasMeter`
            // happened inside a contract execution, such as `host functions`,
            // that currently we don't have.
            // The API will change once we're going to work on VM2 and deciding
            // how to handle the gas consumption inside native calls.
            parent_meter.update(parent_instance, spent)?;
        }
        Ok(self.gas_meter()?.clone())
    }
}
