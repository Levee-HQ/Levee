use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum SettlementError {
    AlreadyInitialized = 1,
    NotAuthorized = 2,
    PolicyNotFound = 3,
    PolicyNotActive = 4,
    TriggerNotMet = 5,
    PolicyExpired = 6,
    PolicyNotExpired = 7,
    InsufficientFunds = 8,
}
