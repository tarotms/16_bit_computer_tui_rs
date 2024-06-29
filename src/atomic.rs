use std::sync::atomic::AtomicUsize;

pub static CLOCK_ITERATIONS: AtomicUsize = AtomicUsize::new(0);
