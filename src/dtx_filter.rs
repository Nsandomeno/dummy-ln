use bitcoin::hash_types::Txid;
use bitcoin::blockdata::script::Script;

use lightning::chain::Filter;
use lightning::chain::WatchedOutput;

pub struct DTxFilter();

impl Filter for DTxFilter {
    fn register_tx(&self, txid: &Txid, script_pubkey: &Script) {
        // <insert code for you to watch for this transaction on-chain>
    }
    fn register_output(&self, output: WatchedOutput) {
        // <insert code for you to watch for any transactions that spend this
        // output on-chain>
    }
}