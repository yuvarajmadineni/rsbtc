use crate::crypto::{PublicKey, Signature};
use crate::sha256::Hash;
use crate::U256;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Blockchain {
    blocks: Vec<Block>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BlockHeader {
    pub timestamp: DateTime<Utc>,  // time when the block is created
    pub nonce: u64,                // number used only once , we increment it to mine the block
    pub prev_block_hash: [u8; 32], // hash of the prev block in the chain
    pub merkle_root: [u8; 32],     // hash of the merkle root derived from all hashes in the block ,
    // ensures all transactions are accounted and unalterable without changing the header
    pub target: U256, // a number which has to be higher than the hash of this block to be
                      // considered valid
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub inputs: Vec<TransactionInput>,
    pub outputs: Vec<TransactionOutput>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransactionInput {
    pub prev_transaction_output_hash: [u8; 32],
    pub signature: Signature,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransactionOutput {
    pub value: u64,
    pub unique_id: Uuid, // ensure hash of each output is unique
    pub pub_key: PublicKey,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain { blocks: vec![] }
    }

    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }
}

impl Block {
    pub fn new(header: BlockHeader, transactions: Vec<Transaction>) -> Self {
        Block {
            header,
            transactions,
        }
    }

    pub fn hash(&self) -> ! {
        unimplemented!()
    }
}

impl BlockHeader {
    pub fn new(
        timestamp: DateTime<Utc>,
        nonce: u64,
        prev_block_hash: [u8; 32],
        merkle_root: [u8; 32],
        target: U256,
    ) -> Self {
        BlockHeader {
            timestamp,
            nonce,
            prev_block_hash,
            merkle_root,
            target,
        }
    }

    pub fn hash(&self) -> Hash {
        Hash::hash(self)
    }
}

impl TransactionOutput {
    pub fn hash(&self) -> Hash {
        Hash::hash(self)
    }
}

impl Transaction {
    pub fn new(inputs: Vec<TransactionInput>, outputs: Vec<TransactionOutput>) -> Self {
        Transaction { inputs, outputs }
    }

    pub fn hash(&self) -> Hash {
        Hash::hash(self)
    }
}
