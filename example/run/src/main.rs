fn main() {
    let code = include_bytes!("../../../target/wasm32-unknown-unknown/release/compile.wasm");

    let wasm_vm = vm::Vm::new();

    let mut gas_meter = vm::GasMeter::with_limit(10000000);
    wasm_vm
        .execute(code, "invoke", &mut gas_meter)
        .expect("execution failed");
}
