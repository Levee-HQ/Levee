use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum PolicyError {
    AlreadyInitialized = 1,
    NotAuthorized = 2,
    InvalidAmount = 3,
    InvalidTerm = 4,
    PerilInactive = 5,
    PolicyNotFound = 6,
    PolicyNotActive = 7,
    Overflow = 8,
}
