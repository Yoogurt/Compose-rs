use crate::foundation::{InnerPlaceable};

impl InnerPlaceable {
    pub(crate) fn new() -> InnerPlaceable {
        InnerPlaceable {
            measure_result: Default::default(),
        }
    }
}