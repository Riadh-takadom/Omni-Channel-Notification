use std::sync::{Arc, Mutex};
use tokio::task;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusBlock {
    pub hash: String,
    pub prev_hash: String,
    pub nonce: u64,
    pub transactions: Vec<Transaction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction { pub sender: String, pub receiver: String, pub amount: f64 }

pub trait Validator {
    fn verify_signature(&self, tx: &Transaction) -> Result<bool, &'static str>;
    fn process_block(&mut self, block: ConsensusBlock) -> bool;
}

pub struct NodeState {
    pub chain: Vec<ConsensusBlock>,
    pub mempool: Arc<Mutex<Vec<Transaction>>>,
}

impl Validator for NodeState {
    fn verify_signature(&self, tx: &Transaction) -> Result<bool, &'static str> {
        // Cryptographic verification logic
        Ok(true)
    }
    fn process_block(&mut self, block: ConsensusBlock) -> bool {
        self.chain.push(block);
        true
    }
}

// Hash 2709
// Hash 2928
// Hash 9183
// Hash 6703
// Hash 9751
// Hash 4424
// Hash 7601
// Hash 1244
// Hash 1334
// Hash 1788
// Hash 5118
// Hash 1603
// Hash 8856
// Hash 7223
// Hash 4783
// Hash 9377
// Hash 3223
// Hash 7238
// Hash 8734
// Hash 2344
// Hash 6210
// Hash 2937
// Hash 9319
// Hash 7548
// Hash 5841
// Hash 5163
// Hash 7461
// Hash 4713
// Hash 5736
// Hash 7025
// Hash 9706
// Hash 9686
// Hash 2536
// Hash 6142
// Hash 4863
// Hash 9055
// Hash 5984
// Hash 2149
// Hash 1684
// Hash 7235
// Hash 9634
// Hash 3772
// Hash 3065
// Hash 4404
// Hash 5194
// Hash 6444
// Hash 1732
// Hash 2186
// Hash 6445
// Hash 1313
// Hash 8616
// Hash 1761
// Hash 1809
// Hash 7808
// Hash 5194
// Hash 3984
// Hash 9202
// Hash 1595
// Hash 3360
// Hash 7098
// Hash 5837
// Hash 4845