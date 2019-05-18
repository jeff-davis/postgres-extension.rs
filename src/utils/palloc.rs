
use crate::utils::memutils::{MemoryContext, CurrentMemoryContext};

pub unsafe fn MemoryContextSwitchTo(context: MemoryContext) -> MemoryContext {
    let old = CurrentMemoryContext;
    CurrentMemoryContext = context;
    return old;
}
