pub mod command;
pub mod error;
pub mod memory;
pub mod types;
pub mod vm;

pub use error::{VmError, Result};
pub use vm::Vm;