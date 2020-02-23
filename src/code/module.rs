use crate::code::instruction::Expression;
use crate::code::types::{MemType, FuncType, ValType};

// https://webassembly.github.io/spec/core/syntax/modules.html
pub struct Module {
    types: Vec<FuncType>,
    funcs: Vec<Function>,
    tables: Vec<Table>,
    mems: Vec<Memory>,
    globals: Vec<Global>,
    elem: Vec<Elem>,
    data: Vec<Data>,
    start: Option<Start>,
    imports: Vec<Import>,
    exports: Vec<Export>
}

pub struct Function {
    type_idx: TypeIndex,
    locals: Vec<ValType>,
    body: Expression
}

// TODO
pub struct Table {

}

pub struct Memory {
    memtype: MemType
}

pub struct Global {
    type_idx: TypeIndex,
    init: Expression
}

// Element segments that initialize a subrange of a table
pub struct Elem {
    table: TableIndex,
    offset: Expression,
    init: Vec<FuncIndex>
}

// Data segments that initialize a subrange of memory
pub struct Data {
    data: MemIndex,
    offset: Expression,
    init: Vec<u8>
}


pub struct Start {
    func: FuncIndex
}

// TODO: Alias String to name
pub struct Import {
    module: String,
    name: String,
    desc: ExportDesc
}

pub enum ImportDesc {
    Func(FuncIndex),
    Table(TableIndex),
    Mem(MemIndex),
    Global(GlobalIndex)
}

// TODO: Alias String to name
pub struct Export {
    name: String,
    desc: ExportDesc
}

pub enum ExportDesc {
    Func(FuncIndex),
    Table(TableIndex),
    Mem(MemIndex),
    Global(GlobalIndex)
}

// Index spaces: https://webassembly.github.io/spec/core/syntax/modules.html#indices
pub type TypeIndex = u32;
pub type FuncIndex = u32;
pub type TableIndex = u32;
pub type MemIndex = u32;
pub type GlobalIndex = u32;
pub type LocalIndex = u32;
pub type LabelIndex = u32;
