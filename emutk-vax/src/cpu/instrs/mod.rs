pub mod operands;
mod instructiontypes;
pub use instructiontypes::*;
mod impls;
pub use impls::execute_instr;