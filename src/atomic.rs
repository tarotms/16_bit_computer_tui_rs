use std::sync::atomic::AtomicUsize;

pub static NAND_CALL_COUNTS: AtomicUsize = AtomicUsize::new(0);
pub static CLOCK_ITERATIONS: AtomicUsize = AtomicUsize::new(0);
