use crate::engine::classifier::Classifier;

#[repr(C, packed)]
pub struct MacroScript {
    _table: u32,
    classifier: Classifier,
    pub op_spec_index: isize,
    _1: u32,
    _2: u32,
    _3: u32,
}
