

// https://webassembly.github.io/spec/core/syntax/types.html#syntax-valtype
pub enum ValType {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64)
}

pub type ResultType = Option<ValType>;

pub struct FuncType {
    parameters: Vec<ValType>,
    results: Vec<ValType>
}

pub struct Limits {
    min: u32,
    max: Option<u32>
}

pub struct MemType {
    limits: Limits
}

pub enum Mut {
    Const,
    Var
}

pub struct GlobalType {
    mutable: Mut,
    valtype: ValType
}
