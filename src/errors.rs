use std::fmt;

use ethers::prelude::{AbiError, ContractError};
use ethers::providers::{Middleware, ProviderError};
use ethers::types::{H160, U256};
use thiserror::Error;
use tokio::task::JoinError;
use uniswap_v3_math::error::UniswapV3MathError;

#[derive(Error, Debug)]
pub enum CFMMError<M>
where
    M: Middleware,
{
    #[error("Middleware error")]
    MiddlewareError(<M as Middleware>::Error),
    #[error("Provider error")]
    ProviderError(#[from] ProviderError),
    #[error("Contract error")]
    ContractError(#[from] ContractError<M>),
    #[error("ABI Codec error")]
    ABICodecError(#[from] AbiError),
    #[error("Eth ABI error")]
    EthABIError(#[from] ethers::abi::Error),
    #[error("Join error")]
    JoinError(#[from] JoinError),
    #[error("Uniswap V3 math error")]
    UniswapV3MathError(#[from] UniswapV3MathError),
    #[error("Pair for token_a/token_b does not exist in provided dexes")]
    PairDoesNotExistInDexes(H160, H160),
    #[error("Could not initialize new pool from event log")]
    UnrecognizedPoolCreatedEventLog,
    #[error("Error when syncing pool")]
    SyncError(H160),
    #[error("Error when getting pool data")]
    PoolDataError,
    #[error("Fixed point math error")]
    FixedPointMathError(#[from] FixedPointMathError),
}

#[derive(Error, Debug)]
pub enum FixedPointMathError {
    ShadowOverflow(U256),
    RoundingError,
    YIsZero,
}

impl std::fmt::Display for FixedPointMathError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "")
    }
}
