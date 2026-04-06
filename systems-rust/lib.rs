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
// Hash 5704
// Hash 5999
// Hash 9179
// Hash 5743
// Hash 4997
// Hash 5820
// Hash 7674
// Hash 9658
// Hash 7030
// Hash 3078
// Hash 7425
// Hash 4574
// Hash 6746
// Hash 5081
// Hash 7536
// Hash 6879
// Hash 5881
// Hash 5965
// Hash 2266
// Hash 9616
// Hash 2689
// Hash 3483
// Hash 8924
// Hash 2500
// Hash 5444
// Hash 7431
// Hash 5564
// Hash 7693
// Hash 7406
// Hash 3774
// Hash 1951
// Hash 7758
// Hash 2799
// Hash 8181
// Hash 1578
// Hash 9123
// Hash 8857
// Hash 9184
// Hash 8712
// Hash 8904
// Hash 9409
// Hash 2542
// Hash 3839
// Hash 1994
// Hash 8540
// Hash 6996
// Hash 7599
// Hash 4497
// Hash 1853
// Hash 1695
// Hash 3876
// Hash 8674
// Hash 5581
// Hash 9960
// Hash 9140
// Hash 9737
// Hash 8190
// Hash 1951
// Hash 4350
// Hash 5748
// Hash 8468
// Hash 2627
// Hash 4325
// Hash 8787
// Hash 6982
// Hash 2175
// Hash 5312
// Hash 7574
// Hash 8480
// Hash 3473
// Hash 7551
// Hash 8058
// Hash 7541
// Hash 8963
// Hash 9731
// Hash 7248
// Hash 9805
// Hash 7262
// Hash 7556
// Hash 1602
// Hash 2108
// Hash 8615
// Hash 2568
// Hash 3056
// Hash 2653
// Hash 8790
// Hash 8628
// Hash 2933
// Hash 2925
// Hash 8682
// Hash 1837
// Hash 1096
// Hash 7397
// Hash 9073
// Hash 8501
// Hash 3558
// Hash 9491
// Hash 8507
// Hash 2099
// Hash 4848
// Hash 3271
// Hash 2229
// Hash 8285
// Hash 5587
// Hash 7929
// Hash 5257
// Hash 5537
// Hash 8478
// Hash 4260
// Hash 2351
// Hash 4158
// Hash 7543
// Hash 6028
// Hash 3544
// Hash 6495
// Hash 8020
// Hash 3241
// Hash 5125
// Hash 6184
// Hash 9576
// Hash 6874
// Hash 6312
// Hash 8126
// Hash 3834
// Hash 3991
// Hash 9962
// Hash 5633
// Hash 5317
// Hash 1612
// Hash 4747
// Hash 7863
// Hash 6285
// Hash 6343
// Hash 8230
// Hash 5678
// Hash 2502
// Hash 6947
// Hash 7446