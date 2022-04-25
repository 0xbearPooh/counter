use cosmwasm_std::{StdError, Uint128};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
    #[error("Overflow")]
    Overflow {},

    #[error("Insufficient Funds. Tried to withdraw {amount:?} but only have {balance:?}")]
    InsufficientFunds { amount: Uint128, balance: Uint128 },
}
