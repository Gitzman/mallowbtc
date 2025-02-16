use thiserror::Error;
use bdk_wallet::descriptor::error::Error as BdkError;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Key error: {0}")]
    KeyError(String),
    
    #[error("Script error: {0}")]
    ScriptError(String),
    
    #[error("Transaction error: {0}")]
    TransactionError(String),
    
    #[error(transparent)]
    BdkError(#[from] BdkError),
}