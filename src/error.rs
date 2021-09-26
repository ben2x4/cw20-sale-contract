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
    #[error("Incorrect funds")]
    IncorretFunds {},

    #[error("Divide by zero error")]
    DivideByZeroError {},

    #[error("Invalid subtraction")]
    SubtractionError {},

    #[error("Price provided is not current")]
    PriceNotCurrentError {
        denom_current: String,
        denom_provided: String,
        price_current: Uint128,
        price_provided: Uint128,
    },
}
