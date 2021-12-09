use crate::bitmap::mask::Masks;
use crate::bitmap::select::SelectLookup8;

/// A container of resources to speed up variant bitmap lookup operations.
pub struct Context {
    pub masks: Masks,
    pub select_lookup_8: SelectLookup8,
}

impl Context {
    pub fn new() -> Self {
        Context {
            masks: Masks::new(),
            select_lookup_8: SelectLookup8::new(),
        }
    }
}
