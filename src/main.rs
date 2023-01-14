// Broadcaster
use crate::dbroadcaster::DBroadcaster;
// Fee Estimator
use crate::dfee_estimator::DFeeEstimator;
// Tx filter
use crate::dtx_filter::DTxFilter;
// Persister
use crate::dpersist::DPersister;
// Logger 
use crate::dlogger::DLogger;
// LDK Dependencies
use lightning::chain::Filter;
use lightning::chain::chainmonitor::ChainMonitor;

{
    let tx_filter     = DTxFilter();
    let store         = DPersister();
    let logger        = DLogger();
    let broadcaster   = DBroadcaster();
    let fee_estimator = DFeeEstimator();

    let filter: Option<Box<dyn Filter>> = Filter;

    let chain_monitor = ChainMonitor::new(
        filter,
        &broadcaster,
        &logger,
        &fee_estimator,
        &store,
    );
}

fn main() {
    println!("hi")
}
