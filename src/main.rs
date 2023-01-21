use std::sync::{Arc};
// Fee Estimator
use lightning::chain::chaininterface::{FeeEstimator, ConfirmationTarget};
// Logger
use time::OffsetDateTime;
use lightning::util::logger::{Logger, Record};
// Broadcast Interface
use bitcoin::blockdata::transaction::Transaction;
use lightning::chain::chaininterface::BroadcasterInterface;
// Persist
use lightning::chain::keysinterface::Sign;
use lightning::chain::transaction::OutPoint;
use lightning::chain::ChannelMonitorUpdateStatus;
use lightning::chain::chainmonitor::{Persist, MonitorUpdateId};
use lightning::chain::channelmonitor::{ChannelMonitor, ChannelMonitorUpdate};
// A part of bitcoind client in ldk-sample
struct DFeeEstimator();
impl FeeEstimator for DFeeEstimator {
	fn get_est_sat_per_1000_weight(
		&self, confirmation_target: ConfirmationTarget,
	) -> u32 {
		match confirmation_target {
			ConfirmationTarget::Background => {3} // fetch background feerate,
			ConfirmationTarget::Normal => {10} // fetch normal feerate (~6 blocks)
			ConfirmationTarget::HighPriority => {15} // fetch high priority feerate
		}
	}
}
// Comes from Disk module in ldk-sample
struct DLogger();
impl Logger for DLogger {
	fn log(&self, record: &Record) {
		let raw_log = record.args.to_string();
		let log = format!(
			"{} {:<5} [{}:{}] {}\n",
			OffsetDateTime::now_utc().unix_timestamp(),
			record.level.to_string(),
			record.module_path,
			record.line,
			raw_log
		);
        // <insert code to print this log and/or write this log to disk>
	}
}
// Comes from Bitcoind Client in ldk-sample
struct DTxBroadcaster();
impl BroadcasterInterface for DTxBroadcaster {
	fn broadcast_transaction(&self, tx: &Transaction) {
        // <insert code to broadcast this transaction>
	}
}
// Comes directly from lightning_persister crate in ldk-sample
struct DPersister();
impl <ChannelSigner: Sign> Persist<ChannelSigner> for DPersister {
    fn persist_new_channel(&self, channel_id: OutPoint, data: &ChannelMonitor<ChannelSigner>, update_id: MonitorUpdateId) -> ChannelMonitorUpdateStatus {
        // <insert code to persist the ChannelMonitor to disk and/or backups>
        // Note that monitor.encode() will get you the ChannelMonitor as a
        // Vec<u8>. 
    }
    fn update_persisted_channel(&self, channel_id: OutPoint, update: &Option<ChannelMonitorUpdate>, data: &ChannelMonitor<ChannelSigner>, update_id: MonitorUpdateId) -> ChannelMonitorUpdateStatus {
        // <insert code to persist either the ChannelMonitor or the
        //  ChannelMonitorUpdate to disk>   
    }
}

async fn start() {
    // equivalent to start_ldk in ldk-sample

    // Setup
    // Step 1 Fee Estimator 
    // Handled by Bitcoind Client in ldk-sample
    // TODO - DFeeEstimator needs to be cloned
    let fee_estimator = Arc::new(DFeeEstimator());
    // Step 2 Logger
    // Handled by Disk Module in ldk-sample
    // TODO - DLogger needs to create a new instance or potentially clone (see ldk-sample: directory is cloned in default impl)
    let logger = Arc::new(DLogger());
    // Step 3 Broadcast Interface
    // Handled by Bitcoind Client in ldk-sample
    // TODO DTxBroadcaster needs to be cloned
    let broadcaster = Arc::new(DTxBroadcaster());
    // Step 4 Persist
    // Handling directly by a ldk struct FileSystemPersister in ldk-sample
    // TODO - See reference in ldk-sample once custom component is developed
    // Should start with default implementation of component
    let persister = Arc::new(DPersister());
    // Step 4 (a) Tx Filter (None for now - Revisit later)
    
}


fn main() {
    println!("hi")
}
