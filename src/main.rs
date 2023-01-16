// Std Lib Dependencies
use std::sync::Arc;
// Broadcaster
pub mod dbroadcaster;
use crate::dbroadcaster::DBroadcaster;
// Fee Estimator
pub mod dfee_estimator;
use crate::dfee_estimator::DFeeEstimator;
// Tx filter - Not required; Using default None for now.
// pub mod dtx_filter;
// use crate::dtx_filter::DTxFilter;
// Persister
pub mod dpersist;
use crate::dpersist::DPersister;// Logger 
pub mod dlogger;
use crate::dlogger::DLogger;
// LDK Dependencies
use lightning::chain::Filter;
use lightning::chain::chainmonitor;
use lightning::chain::keysinterface::{InMemorySigner};

// TODO plug-in the right imported types and components that are playing each role.
// Note this snippet is from the sample which uses a BitcoinDClient to play multiple roles,
// namely, the FeeEstimator and the Broadcaster
type ChainMonitor = chainmonitor::ChainMonitor<
	InMemorySigner, // Channel Signer
	Arc<dyn Filter + Send + Sync>, // Tx Filter (type) - Not requiring using default None for now.
	Arc<DBroadcaster>, // Broadcast Interface (BitcoinD Client) (type)
	Arc<DFeeEstimator>, // Fee Estimator (BitcoinD Client) (type)
	Arc<DLogger>, // Logger (type)
	Arc<DPersister>, // Persister
>;

async fn start() {
    //let filter: Option<Box<dyn Filter>> = Filter; // TODO where should this go? Does it need to be more Global?
    // TODO - Reference line 384 to 426 of start_ldk method in main.rs
    // of the sample implementation: https://github.com/Nsandomeno/ldk-sample/blob/6814a642660772f2859792e188c5273625d1cf57/src/main.rs#L384

    // LDK Step 1 - Fee Estimator (handled by the bitcoin_d client in sample)
    let fee_estimator = DFeeEstimator::new();
    // LDK Step 2 - Logger
    let logger = Arc::new(DLogger::new());
    // LDK Step 3 - Broadcaster Interface (handled by bitcoin_d client in sample)
    let broadcaster = DBroadcaster::new();   
    // LDK Step 4 - Init Persister
    let persister = Arc::new(DPersister::new());
    // LDK Step 5 - (a) Tx Filter 
    //let tx_filter = Arc::new(DTxFilter::new()); - Not required; Ignoring for now.
    // LDK Step 5 TODO - (b) Chain Monitor

}

fn main() {
    println!("hi")
}
