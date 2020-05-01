use std::fmt;

#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ErrorKind {
    /* Arithmetic */
    IntegerOverflow,
    IntegerDivZero,
    DecimalDivZero,
    DecimalOverflow,
    SubscriptRange,
    FloatingOverflow,
    FloatingDivZero,
    FloatingUnderflow,
    /* Memory Management */
    AccessControlViolation,
    TranslationNotValid,
    /* Operand Reference */
    ReservedAddressingMode,
    ReservedOperand,
    /* Instruction Execution */
    ReservedInstruction,
    PrivilegedInstruction,
    OpcodeReservedToCustomers,
    ChangeMode,
    Breakpoint,
    Trace,
    /* Other */
    KernelStackNotValid, // Also known as "F*CK"
    InterruptStackNotValid, // How to give the BOFH a heart attack.
    MachineCheck, // Emulator did an oopsie.
}

#[derive(Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Error {
    kind: ErrorKind,
    data: [u32;2], // Exception specific data.
}

impl Error {
    pub fn new_address_mode_fault() -> Self {
        Error {
            kind: ErrorKind::ReservedAddressingMode,
            data: [0; 2],
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let dispname = match self.kind {
            ErrorKind::IntegerOverflow => "Integer Overflow Trap",
            ErrorKind::IntegerDivZero => "Integer Division by Zero Trap",
            ErrorKind::DecimalDivZero => "Decimal Division by Zero Trap",
            ErrorKind::DecimalOverflow => "Decimal Overflow Trap",
            ErrorKind::SubscriptRange => "Subscript Range Trap",
            ErrorKind::FloatingOverflow => "Floating point Overflow Fault",
            ErrorKind::FloatingDivZero => "Floating point Division by Zero Fault",
            ErrorKind::FloatingUnderflow => "Floating point Underflow Fault",
            ErrorKind::AccessControlViolation => "MMU Access Control Violation Fault",
            ErrorKind::TranslationNotValid => "MMU Page Translation Not Valid Fault",
            ErrorKind::ReservedAddressingMode => "Reserved Instruction Addressing Mode Fault",
            ErrorKind::ReservedOperand => "Reserved Instruction Operand Exception",
            ErrorKind::ReservedInstruction => "Reserved Instruction Fault",
            ErrorKind::PrivilegedInstruction => "Privileged Instruction Fault",
            ErrorKind::OpcodeReservedToCustomers => "Opcode Reserved To Customers Fault",
            ErrorKind::ChangeMode => "Mode Change Trap",
            ErrorKind::Breakpoint => "Breakpoint Fault",
            ErrorKind::Trace => "Trace Fault",
            ErrorKind::KernelStackNotValid => "Kernel Stack Not Valid Abort",
            ErrorKind::InterruptStackNotValid => "Interrupt Stack Not Valid Halt",
            ErrorKind::MachineCheck => "Machine Check Exception",
        };
        write!(f, "VAX System Error: {}", dispname)
    }
}

impl std::error::Error for Error {}