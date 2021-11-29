use crate::mask::Masks;
use crate::select::SelectLookup8;

/// A container of resources to speed up variant bitmap lookup operations.
pub struct LookupTable {
    pub masks: Masks,
    pub select_lookup8: SelectLookup8,
}

impl LookupTable {
    pub fn new() -> Self {
        LookupTable {
            masks: Masks::new(),
            select_lookup8: SelectLookup8::new(),
        }
    }
}
