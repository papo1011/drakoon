#[derive(Debug, Clone)]
pub struct ValueObj {
    pub value: String,
    pub type_item: TypeItem,
}

#[derive(Debug, Clone)]
pub struct TypeItem {
    pub tag: Tag,
    pub llvm_type: LLVMType,
    pub align: usize,
    pub size: Option<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Tag {
    Base,
    Array,
    Immediate,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LLVMType {
    I32,
    Double,
    Void,
    Array(Box<LLVMType>, usize),
}
