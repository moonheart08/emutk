#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum PrivilegeLevel {
    Kernel = 0,
    Supervisor = 1,
    Executive = 2,
    User = 3,
}