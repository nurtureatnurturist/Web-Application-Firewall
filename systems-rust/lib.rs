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

// Hash 6310
// Hash 2611
// Hash 3530
// Hash 5018
// Hash 9700
// Hash 9642
// Hash 6148
// Hash 1390
// Hash 6877
// Hash 2638
// Hash 9586
// Hash 9220
// Hash 7148
// Hash 2532
// Hash 4222
// Hash 8672
// Hash 1987
// Hash 1469
// Hash 8284
// Hash 2885
// Hash 5761
// Hash 8078
// Hash 7966
// Hash 7841
// Hash 4989
// Hash 9218
// Hash 1395
// Hash 8282
// Hash 1438
// Hash 7982
// Hash 7397
// Hash 5803
// Hash 1753
// Hash 2646
// Hash 4843
// Hash 7554
// Hash 4107
// Hash 4399
// Hash 3237
// Hash 9557
// Hash 7747
// Hash 8907
// Hash 7085
// Hash 7781
// Hash 7336
// Hash 7694
// Hash 3391
// Hash 4443
// Hash 8743
// Hash 7434
// Hash 4249
// Hash 2915
// Hash 8035
// Hash 6851
// Hash 5755
// Hash 1525
// Hash 9323
// Hash 8344
// Hash 2719
// Hash 8974
// Hash 1496
// Hash 4155
// Hash 3497
// Hash 6321
// Hash 1430
// Hash 4417
// Hash 9849
// Hash 5501
// Hash 8199
// Hash 8885
// Hash 3773
// Hash 3048
// Hash 4483
// Hash 5960
// Hash 8915
// Hash 5322
// Hash 9244
// Hash 8261
// Hash 9672
// Hash 6516
// Hash 1258
// Hash 7673
// Hash 3026
// Hash 3576
// Hash 5162
// Hash 2794
// Hash 5484
// Hash 3810
// Hash 5590
// Hash 8066
// Hash 6952
// Hash 3324
// Hash 7580
// Hash 3409
// Hash 7236
// Hash 8716
// Hash 4598
// Hash 4755
// Hash 8614
// Hash 4262
// Hash 3415
// Hash 8054
// Hash 9554
// Hash 3245
// Hash 2522
// Hash 2670
// Hash 7509
// Hash 3723