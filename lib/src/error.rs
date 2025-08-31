use thiserror::Error;

#[derive(Error, Debug)]
pub enum BtcError {
    #[error("Invalid Transaction")]
    InvalidTransaction,
    #[error("Invalid Block")]
    InvalidBlock,
    #[error("Invalid Block header")]
    InvalidBlockHeader,
    #[error("Invalid transaction input")]
    InvalidTransactionInput,
    #[error("Invalid transaction output")]
    InvalidTransactionOutput,
    #[error("Invalid Merkle root")]
    InvalidMerkleRoot,
    #[error("Invalid hash")]
    InvalidHash,
    #[error("Invalid signature")]
    InvalidSignature,
    #[error("Invalid public key")]
    InvalidPublicKey,
    #[error("Invalid private key")]
    InvalidPrivateKey,
}

pub type Result<T> = std::result::Result<T, BtcError>;
