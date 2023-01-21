use lightning::chain::keysinterface::Sign;
use lightning::chain::transaction::OutPoint;
use lightning::chain::ChannelMonitorUpdateStatus;
use lightning::chain::chainmonitor::{Persist, MonitorUpdateId};
use lightning::chain::channelmonitor::{ChannelMonitor, ChannelMonitorUpdate};

pub struct DPersister();

impl DPersister {
    pub fn new() -> DPersister {
        DPersister()
    }
}

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