use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

pub mod alignment_lib;
pub mod reference;
pub mod wavefront_alignment;
