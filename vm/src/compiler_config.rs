// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

use crate::config::Config;

use std::sync::Arc;

use wasmer::wasmparser::Operator;
use wasmer::wasmparser::Operator::*;
use wasmer::CompilerConfig;
use wasmer_compiler_singlepass::Singlepass;
use wasmer_middlewares::Metering;

pub struct CompilerConfigProvider;

impl CompilerConfigProvider {
    pub fn singlepass(config: &'static Config) -> Singlepass {
        let cost_function = move |operator: &Operator| -> u64 {
            match operator {
                Unreachable => config.op_costs.unreachable,
                Nop => config.op_costs.nop,
                Block { .. } => config.op_costs.flow,
                Loop { .. } => config.op_costs.flow,
                If { .. } => config.op_costs.flow,
                Else => config.op_costs.flow,
                Try { .. } => config.op_costs.flow,
                Catch { .. } => config.op_costs.flow,
                Throw { .. } => config.op_costs.flow,
                Rethrow { .. } => config.op_costs.flow,
                End => config.op_costs.flow,
                Br { .. } => config.op_costs.flow,
                BrIf { .. } => config.op_costs.flow,
                BrTable { .. } => config.op_costs.flow,
                Return => config.op_costs.flow,
                Call { .. } => config.op_costs.flow,
                CallIndirect { .. } => config.op_costs.flow,
                ReturnCall { .. } => config.op_costs.flow,
                ReturnCallIndirect { .. } => config.op_costs.flow,
                Delegate { .. } => config.op_costs.flow,
                CatchAll => config.op_costs.flow,
                Drop => config.op_costs.flow,
                Select => config.op_costs.flow,
                TypedSelect { .. } => config.op_costs.flow,
                LocalGet { .. } => config.op_costs.local,
                LocalSet { .. } => config.op_costs.local,
                LocalTee { .. } => config.op_costs.local,
                GlobalGet { .. } => config.op_costs.global,
                GlobalSet { .. } => config.op_costs.global,
                I32Load { .. } => config.op_costs.load,
                I64Load { .. } => config.op_costs.load,
                F32Load { .. } => config.op_costs.load,
                F64Load { .. } => config.op_costs.load,
                I32Load8S { .. } => config.op_costs.load,
                I32Load8U { .. } => config.op_costs.load,
                I32Load16S { .. } => config.op_costs.load,
                I32Load16U { .. } => config.op_costs.load,
                I64Load8S { .. } => config.op_costs.load,
                I64Load8U { .. } => config.op_costs.load,
                I64Load16S { .. } => config.op_costs.load,
                I64Load16U { .. } => config.op_costs.load,
                I64Load32S { .. } => config.op_costs.load,
                I64Load32U { .. } => config.op_costs.load,
                I32Store { .. } => config.op_costs.store,
                I64Store { .. } => config.op_costs.store,
                F32Store { .. } => config.op_costs.store,
                F64Store { .. } => config.op_costs.store,
                I32Store8 { .. } => config.op_costs.store,
                I32Store16 { .. } => config.op_costs.store,
                I64Store8 { .. } => config.op_costs.store,
                I64Store16 { .. } => config.op_costs.store,
                I64Store32 { .. } => config.op_costs.store,
                MemorySize { .. } => config.op_costs.current_mem,
                MemoryGrow { .. } => config.op_costs.grow_mem,
                I32Const { .. } => config.op_costs.const_decl,
                I64Const { .. } => config.op_costs.const_decl,
                F32Const { .. } => config.regular_op_cost,
                F64Const { .. } => config.regular_op_cost,
                RefNull { .. } => config.regular_op_cost,
                RefIsNull => config.regular_op_cost,
                RefFunc { .. } => config.regular_op_cost,
                I32Eqz => config.op_costs.integer_comp,
                I32Eq => config.op_costs.integer_comp,
                I32Ne => config.op_costs.integer_comp,
                I32LtS => config.op_costs.integer_comp,
                I32LtU => config.op_costs.integer_comp,
                I32GtS => config.op_costs.integer_comp,
                I32GtU => config.op_costs.integer_comp,
                I32LeS => config.op_costs.integer_comp,
                I32LeU => config.op_costs.integer_comp,
                I32GeS => config.op_costs.integer_comp,
                I32GeU => config.op_costs.integer_comp,
                I64Eqz => config.op_costs.integer_comp,
                I64Eq => config.op_costs.integer_comp,
                I64Ne => config.op_costs.integer_comp,
                I64LtS => config.op_costs.integer_comp,
                I64LtU => config.op_costs.integer_comp,
                I64GtS => config.op_costs.integer_comp,
                I64GtU => config.op_costs.integer_comp,
                I64LeS => config.op_costs.integer_comp,
                I64LeU => config.op_costs.integer_comp,
                I64GeS => config.op_costs.integer_comp,
                I64GeU => config.op_costs.integer_comp,
                F32Eq => config.op_costs.float_comp,
                F32Ne => config.op_costs.float_comp,
                F32Lt => config.op_costs.float_comp,
                F32Gt => config.op_costs.float_comp,
                F32Le => config.op_costs.float_comp,
                F32Ge => config.op_costs.float_comp,
                F64Eq => config.op_costs.float_comp,
                F64Ne => config.op_costs.float_comp,
                F64Lt => config.op_costs.float_comp,
                F64Gt => config.op_costs.float_comp,
                F64Le => config.op_costs.float_comp,
                F64Ge => config.op_costs.float_comp,
                I32Clz => config.op_costs.bit,
                I32Ctz => config.op_costs.bit,
                I32Popcnt => config.op_costs.bit,
                I32Add => config.op_costs.add,
                I32Sub => config.op_costs.add,
                I32Mul => config.op_costs.mul,
                I32DivS => config.op_costs.div,
                I32DivU => config.op_costs.div,
                I32RemS => config.op_costs.div,
                I32RemU => config.op_costs.div,
                I32And => config.op_costs.bit,
                I32Or => config.op_costs.bit,
                I32Xor => config.op_costs.bit,
                I32Shl => config.op_costs.bit,
                I32ShrS => config.op_costs.bit,
                I32ShrU => config.op_costs.bit,
                I32Rotl => config.op_costs.bit,
                I32Rotr => config.op_costs.bit,
                I64Clz => config.op_costs.bit,
                I64Ctz => config.op_costs.bit,
                I64Popcnt => config.op_costs.bit,
                I64Add => config.op_costs.add,
                I64Sub => config.op_costs.add,
                I64Mul => config.op_costs.mul,
                I64DivS => config.op_costs.div,
                I64DivU => config.op_costs.div,
                I64RemS => config.op_costs.div,
                I64RemU => config.op_costs.div,
                I64And => config.op_costs.bit,
                I64Or => config.op_costs.bit,
                I64Xor => config.op_costs.bit,
                I64Shl => config.op_costs.bit,
                I64ShrS => config.op_costs.bit,
                I64ShrU => config.op_costs.bit,
                I64Rotl => config.op_costs.bit,
                I64Rotr => config.op_costs.bit,
                F32Abs => config.op_costs.float,
                F32Neg => config.op_costs.float,
                F32Ceil => config.op_costs.float,
                F32Floor => config.op_costs.float,
                F32Trunc => config.op_costs.float,
                F32Nearest => config.op_costs.float,
                F32Sqrt => config.op_costs.float,
                F32Add => config.op_costs.float,
                F32Sub => config.op_costs.float,
                F32Mul => config.op_costs.float,
                F32Div => config.op_costs.float,
                F32Min => config.op_costs.float,
                F32Max => config.op_costs.float,
                F32Copysign => config.op_costs.float,
                F64Abs => config.op_costs.float,
                F64Neg => config.op_costs.float,
                F64Ceil => config.op_costs.float,
                F64Floor => config.op_costs.float,
                F64Trunc => config.op_costs.float,
                F64Nearest => config.op_costs.float,
                F64Sqrt => config.op_costs.float,
                F64Add => config.op_costs.float,
                F64Sub => config.op_costs.float,
                F64Mul => config.op_costs.float,
                F64Div => config.op_costs.float,
                F64Min => config.op_costs.float,
                F64Max => config.op_costs.float,
                F64Copysign => config.op_costs.float,
                I32WrapI64 => config.op_costs.conversion,
                I64ExtendI32S => config.op_costs.conversion,
                I64ExtendI32U => config.op_costs.conversion,
                I32TruncF32S => config.op_costs.float_conversion,
                I32TruncF32U => config.op_costs.float_conversion,
                I32TruncF64S => config.op_costs.float_conversion,
                I32TruncF64U => config.op_costs.float_conversion,
                I64TruncF32S => config.op_costs.float_conversion,
                I64TruncF32U => config.op_costs.float_conversion,
                I64TruncF64S => config.op_costs.float_conversion,
                I64TruncF64U => config.op_costs.float_conversion,
                F32ConvertI32S => config.op_costs.float_conversion,
                F32ConvertI32U => config.op_costs.float_conversion,
                F32ConvertI64S => config.op_costs.float_conversion,
                F32ConvertI64U => config.op_costs.float_conversion,
                F32DemoteF64 => config.op_costs.float_conversion,
                F64ConvertI32S => config.op_costs.float_conversion,
                F64ConvertI32U => config.op_costs.float_conversion,
                F64ConvertI64S => config.op_costs.float_conversion,
                F64ConvertI64U => config.op_costs.float_conversion,
                F64PromoteF32 => config.op_costs.float_conversion,
                I32ReinterpretF32 => config.op_costs.reinterpret,
                I64ReinterpretF64 => config.op_costs.reinterpret,
                F32ReinterpretI32 => config.op_costs.reinterpret,
                F64ReinterpretI64 => config.op_costs.reinterpret,
                I32Extend8S => config.regular_op_cost,
                I32Extend16S => config.regular_op_cost,
                I64Extend8S => config.regular_op_cost,
                I64Extend16S => config.regular_op_cost,
                I64Extend32S => config.regular_op_cost,

                // 0xFC operators
                // Non-trapping Float-to-int Conversions
                I32TruncSatF32S => config.regular_op_cost,
                I32TruncSatF32U => config.regular_op_cost,
                I32TruncSatF64S => config.regular_op_cost,
                I32TruncSatF64U => config.regular_op_cost,
                I64TruncSatF32S => config.regular_op_cost,
                I64TruncSatF32U => config.regular_op_cost,
                I64TruncSatF64S => config.regular_op_cost,
                I64TruncSatF64U => config.regular_op_cost,

                // 0xFC operators
                // bulk memory https://github.com/WebAssembly/bulk-memory-operations/blob/master/proposals/bulk-memory-operations/Overview.md
                MemoryInit { .. } => config.regular_op_cost,
                DataDrop { .. } => config.regular_op_cost,
                MemoryCopy { .. } => config.regular_op_cost,
                MemoryFill { .. } => config.regular_op_cost,
                TableInit { .. } => config.regular_op_cost,
                ElemDrop { .. } => config.regular_op_cost,
                TableCopy { .. } => config.regular_op_cost,
                TableFill { .. } => config.regular_op_cost,
                TableGet { .. } => config.regular_op_cost,
                TableSet { .. } => config.regular_op_cost,
                TableGrow { .. } => config.regular_op_cost,
                TableSize { .. } => config.regular_op_cost,

                // 0xFE operators
                // https://github.com/WebAssembly/threads/blob/master/proposals/threads/Overview.md
                MemoryAtomicNotify { .. } => config.regular_op_cost,
                MemoryAtomicWait32 { .. } => config.regular_op_cost,
                MemoryAtomicWait64 { .. } => config.regular_op_cost,
                AtomicFence { .. } => config.regular_op_cost,
                I32AtomicLoad { .. } => config.regular_op_cost,
                I64AtomicLoad { .. } => config.regular_op_cost,
                I32AtomicLoad8U { .. } => config.regular_op_cost,
                I32AtomicLoad16U { .. } => config.regular_op_cost,
                I64AtomicLoad8U { .. } => config.regular_op_cost,
                I64AtomicLoad16U { .. } => config.regular_op_cost,
                I64AtomicLoad32U { .. } => config.regular_op_cost,
                I32AtomicStore { .. } => config.regular_op_cost,
                I64AtomicStore { .. } => config.regular_op_cost,
                I32AtomicStore8 { .. } => config.regular_op_cost,
                I32AtomicStore16 { .. } => config.regular_op_cost,
                I64AtomicStore8 { .. } => config.regular_op_cost,
                I64AtomicStore16 { .. } => config.regular_op_cost,
                I64AtomicStore32 { .. } => config.regular_op_cost,
                I32AtomicRmwAdd { .. } => config.regular_op_cost,
                I64AtomicRmwAdd { .. } => config.regular_op_cost,
                I32AtomicRmw8AddU { .. } => config.regular_op_cost,
                I32AtomicRmw16AddU { .. } => config.regular_op_cost,
                I64AtomicRmw8AddU { .. } => config.regular_op_cost,
                I64AtomicRmw16AddU { .. } => config.regular_op_cost,
                I64AtomicRmw32AddU { .. } => config.regular_op_cost,
                I32AtomicRmwSub { .. } => config.regular_op_cost,
                I64AtomicRmwSub { .. } => config.regular_op_cost,
                I32AtomicRmw8SubU { .. } => config.regular_op_cost,
                I32AtomicRmw16SubU { .. } => config.regular_op_cost,
                I64AtomicRmw8SubU { .. } => config.regular_op_cost,
                I64AtomicRmw16SubU { .. } => config.regular_op_cost,
                I64AtomicRmw32SubU { .. } => config.regular_op_cost,
                I32AtomicRmwAnd { .. } => config.regular_op_cost,
                I64AtomicRmwAnd { .. } => config.regular_op_cost,
                I32AtomicRmw8AndU { .. } => config.regular_op_cost,
                I32AtomicRmw16AndU { .. } => config.regular_op_cost,
                I64AtomicRmw8AndU { .. } => config.regular_op_cost,
                I64AtomicRmw16AndU { .. } => config.regular_op_cost,
                I64AtomicRmw32AndU { .. } => config.regular_op_cost,
                I32AtomicRmwOr { .. } => config.regular_op_cost,
                I64AtomicRmwOr { .. } => config.regular_op_cost,
                I32AtomicRmw8OrU { .. } => config.regular_op_cost,
                I32AtomicRmw16OrU { .. } => config.regular_op_cost,
                I64AtomicRmw8OrU { .. } => config.regular_op_cost,
                I64AtomicRmw16OrU { .. } => config.regular_op_cost,
                I64AtomicRmw32OrU { .. } => config.regular_op_cost,
                I32AtomicRmwXor { .. } => config.regular_op_cost,
                I64AtomicRmwXor { .. } => config.regular_op_cost,
                I32AtomicRmw8XorU { .. } => config.regular_op_cost,
                I32AtomicRmw16XorU { .. } => config.regular_op_cost,
                I64AtomicRmw8XorU { .. } => config.regular_op_cost,
                I64AtomicRmw16XorU { .. } => config.regular_op_cost,
                I64AtomicRmw32XorU { .. } => config.regular_op_cost,
                I32AtomicRmwXchg { .. } => config.regular_op_cost,
                I64AtomicRmwXchg { .. } => config.regular_op_cost,
                I32AtomicRmw8XchgU { .. } => config.regular_op_cost,
                I32AtomicRmw16XchgU { .. } => config.regular_op_cost,
                I64AtomicRmw8XchgU { .. } => config.regular_op_cost,
                I64AtomicRmw16XchgU { .. } => config.regular_op_cost,
                I64AtomicRmw32XchgU { .. } => config.regular_op_cost,
                I32AtomicRmwCmpxchg { .. } => config.regular_op_cost,
                I64AtomicRmwCmpxchg { .. } => config.regular_op_cost,
                I32AtomicRmw8CmpxchgU { .. } => config.regular_op_cost,
                I32AtomicRmw16CmpxchgU { .. } => config.regular_op_cost,
                I64AtomicRmw8CmpxchgU { .. } => config.regular_op_cost,
                I64AtomicRmw16CmpxchgU { .. } => config.regular_op_cost,
                I64AtomicRmw32CmpxchgU { .. } => config.regular_op_cost,

                // 0xFD operators
                // SIMD https://webassembly.github.io/simd/core/binary/instructions.html
                V128Load { .. } => config.regular_op_cost,
                V128Load8x8S { .. } => config.regular_op_cost,
                V128Load8x8U { .. } => config.regular_op_cost,
                V128Load16x4S { .. } => config.regular_op_cost,
                V128Load16x4U { .. } => config.regular_op_cost,
                V128Load32x2S { .. } => config.regular_op_cost,
                V128Load32x2U { .. } => config.regular_op_cost,
                V128Load8Splat { .. } => config.regular_op_cost,
                V128Load16Splat { .. } => config.regular_op_cost,
                V128Load32Splat { .. } => config.regular_op_cost,
                V128Load64Splat { .. } => config.regular_op_cost,
                V128Load32Zero { .. } => config.regular_op_cost,
                V128Load64Zero { .. } => config.regular_op_cost,
                V128Store { .. } => config.regular_op_cost,
                V128Load8Lane { .. } => config.regular_op_cost,
                V128Load16Lane { .. } => config.regular_op_cost,
                V128Load32Lane { .. } => config.regular_op_cost,
                V128Load64Lane { .. } => config.regular_op_cost,
                V128Store8Lane { .. } => config.regular_op_cost,
                V128Store16Lane { .. } => config.regular_op_cost,
                V128Store32Lane { .. } => config.regular_op_cost,
                V128Store64Lane { .. } => config.regular_op_cost,
                V128Const { .. } => config.regular_op_cost,
                I8x16Shuffle { .. } => config.regular_op_cost,
                I8x16ExtractLaneS { .. } => config.regular_op_cost,
                I8x16ExtractLaneU { .. } => config.regular_op_cost,
                I8x16ReplaceLane { .. } => config.regular_op_cost,
                I16x8ExtractLaneS { .. } => config.regular_op_cost,
                I16x8ExtractLaneU { .. } => config.regular_op_cost,
                I16x8ReplaceLane { .. } => config.regular_op_cost,
                I32x4ExtractLane { .. } => config.regular_op_cost,
                I32x4ReplaceLane { .. } => config.regular_op_cost,
                I64x2ExtractLane { .. } => config.regular_op_cost,
                I64x2ReplaceLane { .. } => config.regular_op_cost,
                F32x4ExtractLane { .. } => config.regular_op_cost,
                F32x4ReplaceLane { .. } => config.regular_op_cost,
                F64x2ExtractLane { .. } => config.regular_op_cost,
                F64x2ReplaceLane { .. } => config.regular_op_cost,
                I8x16Swizzle => config.regular_op_cost,
                I8x16Splat => config.regular_op_cost,
                I16x8Splat => config.regular_op_cost,
                I32x4Splat => config.regular_op_cost,
                I64x2Splat => config.regular_op_cost,
                F32x4Splat => config.regular_op_cost,
                F64x2Splat => config.regular_op_cost,
                I8x16Eq => config.regular_op_cost,
                I8x16Ne => config.regular_op_cost,
                I8x16LtS => config.regular_op_cost,
                I8x16LtU => config.regular_op_cost,
                I8x16GtS => config.regular_op_cost,
                I8x16GtU => config.regular_op_cost,
                I8x16LeS => config.regular_op_cost,
                I8x16LeU => config.regular_op_cost,
                I8x16GeS => config.regular_op_cost,
                I8x16GeU => config.regular_op_cost,
                I16x8Eq => config.regular_op_cost,
                I16x8Ne => config.regular_op_cost,
                I16x8LtS => config.regular_op_cost,
                I16x8LtU => config.regular_op_cost,
                I16x8GtS => config.regular_op_cost,
                I16x8GtU => config.regular_op_cost,
                I16x8LeS => config.regular_op_cost,
                I16x8LeU => config.regular_op_cost,
                I16x8GeS => config.regular_op_cost,
                I16x8GeU => config.regular_op_cost,
                I32x4Eq => config.regular_op_cost,
                I32x4Ne => config.regular_op_cost,
                I32x4LtS => config.regular_op_cost,
                I32x4LtU => config.regular_op_cost,
                I32x4GtS => config.regular_op_cost,
                I32x4GtU => config.regular_op_cost,
                I32x4LeS => config.regular_op_cost,
                I32x4LeU => config.regular_op_cost,
                I32x4GeS => config.regular_op_cost,
                I32x4GeU => config.regular_op_cost,
                I64x2Eq => config.regular_op_cost,
                I64x2Ne => config.regular_op_cost,
                I64x2LtS => config.regular_op_cost,
                I64x2GtS => config.regular_op_cost,
                I64x2LeS => config.regular_op_cost,
                I64x2GeS => config.regular_op_cost,
                F32x4Eq => config.regular_op_cost,
                F32x4Ne => config.regular_op_cost,
                F32x4Lt => config.regular_op_cost,
                F32x4Gt => config.regular_op_cost,
                F32x4Le => config.regular_op_cost,
                F32x4Ge => config.regular_op_cost,
                F64x2Eq => config.regular_op_cost,
                F64x2Ne => config.regular_op_cost,
                F64x2Lt => config.regular_op_cost,
                F64x2Gt => config.regular_op_cost,
                F64x2Le => config.regular_op_cost,
                F64x2Ge => config.regular_op_cost,
                V128Not => config.regular_op_cost,
                V128And => config.regular_op_cost,
                V128AndNot => config.regular_op_cost,
                V128Or => config.regular_op_cost,
                V128Xor => config.regular_op_cost,
                V128Bitselect => config.regular_op_cost,
                V128AnyTrue => config.regular_op_cost,
                I8x16Abs => config.regular_op_cost,
                I8x16Neg => config.regular_op_cost,
                I8x16Popcnt => config.regular_op_cost,
                I8x16AllTrue => config.regular_op_cost,
                I8x16Bitmask => config.regular_op_cost,
                I8x16NarrowI16x8S => config.regular_op_cost,
                I8x16NarrowI16x8U => config.regular_op_cost,
                I8x16Shl => config.regular_op_cost,
                I8x16ShrS => config.regular_op_cost,
                I8x16ShrU => config.regular_op_cost,
                I8x16Add => config.regular_op_cost,
                I8x16AddSatS => config.regular_op_cost,
                I8x16AddSatU => config.regular_op_cost,
                I8x16Sub => config.regular_op_cost,
                I8x16SubSatS => config.regular_op_cost,
                I8x16SubSatU => config.regular_op_cost,
                I8x16MinS => config.regular_op_cost,
                I8x16MinU => config.regular_op_cost,
                I8x16MaxS => config.regular_op_cost,
                I8x16MaxU => config.regular_op_cost,
                I8x16RoundingAverageU => config.regular_op_cost,
                I16x8ExtAddPairwiseI8x16S => config.regular_op_cost,
                I16x8ExtAddPairwiseI8x16U => config.regular_op_cost,
                I16x8Abs => config.regular_op_cost,
                I16x8Neg => config.regular_op_cost,
                I16x8Q15MulrSatS => config.regular_op_cost,
                I16x8AllTrue => config.regular_op_cost,
                I16x8Bitmask => config.regular_op_cost,
                I16x8NarrowI32x4S => config.regular_op_cost,
                I16x8NarrowI32x4U => config.regular_op_cost,
                I16x8ExtendLowI8x16S => config.regular_op_cost,
                I16x8ExtendHighI8x16S => config.regular_op_cost,
                I16x8ExtendLowI8x16U => config.regular_op_cost,
                I16x8ExtendHighI8x16U => config.regular_op_cost,
                I16x8Shl => config.regular_op_cost,
                I16x8ShrS => config.regular_op_cost,
                I16x8ShrU => config.regular_op_cost,
                I16x8Add => config.regular_op_cost,
                I16x8AddSatS => config.regular_op_cost,
                I16x8AddSatU => config.regular_op_cost,
                I16x8Sub => config.regular_op_cost,
                I16x8SubSatS => config.regular_op_cost,
                I16x8SubSatU => config.regular_op_cost,
                I16x8Mul => config.regular_op_cost,
                I16x8MinS => config.regular_op_cost,
                I16x8MinU => config.regular_op_cost,
                I16x8MaxS => config.regular_op_cost,
                I16x8MaxU => config.regular_op_cost,
                I16x8RoundingAverageU => config.regular_op_cost,
                I16x8ExtMulLowI8x16S => config.regular_op_cost,
                I16x8ExtMulHighI8x16S => config.regular_op_cost,
                I16x8ExtMulLowI8x16U => config.regular_op_cost,
                I16x8ExtMulHighI8x16U => config.regular_op_cost,
                I32x4ExtAddPairwiseI16x8S => config.regular_op_cost,
                I32x4ExtAddPairwiseI16x8U => config.regular_op_cost,
                I32x4Abs => config.regular_op_cost,
                I32x4Neg => config.regular_op_cost,
                I32x4AllTrue => config.regular_op_cost,
                I32x4Bitmask => config.regular_op_cost,
                I32x4ExtendLowI16x8S => config.regular_op_cost,
                I32x4ExtendHighI16x8S => config.regular_op_cost,
                I32x4ExtendLowI16x8U => config.regular_op_cost,
                I32x4ExtendHighI16x8U => config.regular_op_cost,
                I32x4Shl => config.regular_op_cost,
                I32x4ShrS => config.regular_op_cost,
                I32x4ShrU => config.regular_op_cost,
                I32x4Add => config.regular_op_cost,
                I32x4Sub => config.regular_op_cost,
                I32x4Mul => config.regular_op_cost,
                I32x4MinS => config.regular_op_cost,
                I32x4MinU => config.regular_op_cost,
                I32x4MaxS => config.regular_op_cost,
                I32x4MaxU => config.regular_op_cost,
                I32x4DotI16x8S => config.regular_op_cost,
                I32x4ExtMulLowI16x8S => config.regular_op_cost,
                I32x4ExtMulHighI16x8S => config.regular_op_cost,
                I32x4ExtMulLowI16x8U => config.regular_op_cost,
                I32x4ExtMulHighI16x8U => config.regular_op_cost,
                I64x2Abs => config.regular_op_cost,
                I64x2Neg => config.regular_op_cost,
                I64x2AllTrue => config.regular_op_cost,
                I64x2Bitmask => config.regular_op_cost,
                I64x2ExtendLowI32x4S => config.regular_op_cost,
                I64x2ExtendHighI32x4S => config.regular_op_cost,
                I64x2ExtendLowI32x4U => config.regular_op_cost,
                I64x2ExtendHighI32x4U => config.regular_op_cost,
                I64x2Shl => config.regular_op_cost,
                I64x2ShrS => config.regular_op_cost,
                I64x2ShrU => config.regular_op_cost,
                I64x2Add => config.regular_op_cost,
                I64x2Sub => config.regular_op_cost,
                I64x2Mul => config.regular_op_cost,
                I64x2ExtMulLowI32x4S => config.regular_op_cost,
                I64x2ExtMulHighI32x4S => config.regular_op_cost,
                I64x2ExtMulLowI32x4U => config.regular_op_cost,
                I64x2ExtMulHighI32x4U => config.regular_op_cost,
                F32x4Ceil => config.regular_op_cost,
                F32x4Floor => config.regular_op_cost,
                F32x4Trunc => config.regular_op_cost,
                F32x4Nearest => config.regular_op_cost,
                F32x4Abs => config.regular_op_cost,
                F32x4Neg => config.regular_op_cost,
                F32x4Sqrt => config.regular_op_cost,
                F32x4Add => config.regular_op_cost,
                F32x4Sub => config.regular_op_cost,
                F32x4Mul => config.regular_op_cost,
                F32x4Div => config.regular_op_cost,
                F32x4Min => config.regular_op_cost,
                F32x4Max => config.regular_op_cost,
                F32x4PMin => config.regular_op_cost,
                F32x4PMax => config.regular_op_cost,
                F64x2Ceil => config.regular_op_cost,
                F64x2Floor => config.regular_op_cost,
                F64x2Trunc => config.regular_op_cost,
                F64x2Nearest => config.regular_op_cost,
                F64x2Abs => config.regular_op_cost,
                F64x2Neg => config.regular_op_cost,
                F64x2Sqrt => config.regular_op_cost,
                F64x2Add => config.regular_op_cost,
                F64x2Sub => config.regular_op_cost,
                F64x2Mul => config.regular_op_cost,
                F64x2Div => config.regular_op_cost,
                F64x2Min => config.regular_op_cost,
                F64x2Max => config.regular_op_cost,
                F64x2PMin => config.regular_op_cost,
                F64x2PMax => config.regular_op_cost,
                I32x4TruncSatF32x4S => config.regular_op_cost,
                I32x4TruncSatF32x4U => config.regular_op_cost,
                F32x4ConvertI32x4S => config.regular_op_cost,
                F32x4ConvertI32x4U => config.regular_op_cost,
                I32x4TruncSatF64x2SZero => config.regular_op_cost,
                I32x4TruncSatF64x2UZero => config.regular_op_cost,
                F64x2ConvertLowI32x4S => config.regular_op_cost,
                F64x2ConvertLowI32x4U => config.regular_op_cost,
                F32x4DemoteF64x2Zero => config.regular_op_cost,
                F64x2PromoteLowF32x4 => config.regular_op_cost,
                _ => config.regular_op_cost,
            }
        };

        let mut compiler_config = Singlepass::default();
        if config.has_metering {
            let metering = Arc::new(Metering::new(0, cost_function));
            compiler_config.push_middleware(metering);
        } else {
            let metering = Arc::new(Metering::new(0, |_| 0));
            compiler_config.push_middleware(metering);
        }
        compiler_config
    }
}