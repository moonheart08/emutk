pub mod operands;
mod instructiontypes;
pub use instructiontypes::*;
mod impls;
pub use impls::execute_instr;
pub use impls::MultiInstruction;
pub use impls::exec_multi_instructions;