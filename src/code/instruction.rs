use crate::code::module::{LocalIndex, GlobalIndex, LabelIndex, FuncIndex, TypeIndex};
use crate::code::types::ResultType;

// WebAssembly code consists of sequences of instructions.
// https://webassembly.github.io/spec/core/syntax/instructions.html
pub enum Instruction {
    // Numeric instructions
    ConstI32(i32),
    ConstI64(i64),
    ConstF32(f32),
    ConstF64(f64),
    // i32.iunop
    // i64.iunop
    // f32.funop
    // f64.funop
    // i32.ibinop
    // i64.ibinop
    // f32.fbinop
    // f64.fbinop


    // Parametric instructions
    Drop,
    Select,

    // Variable instructions
    LocalGet(LocalIndex),
    LocalSet(LocalIndex),
    LocalTee(LocalIndex),
    GlobalGet(GlobalIndex),
    GlobalSet(GlobalIndex),

    // Memory instructions

    // Control instructions
    Nop,
    Unreachable,
    Block(ResultType, Vec<Instruction>),
    Loop(ResultType, Vec<Instruction>),
    If(ResultType, Vec<Instruction>),
    IfElse(ResultType, Vec<Instruction>, Vec<Instruction>),
    Branch(LabelIndex),
    BranchIf(LabelIndex),
    BranchTable(), // TODO
    Return,
    Call(FuncIndex),
    CallIndirect(TypeIndex)
}

pub struct Expression {
    instructions: Vec<Instruction>
}
