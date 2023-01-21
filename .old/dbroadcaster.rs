use bitcoin::blockdata::transaction::Transaction;
use lightning::chain::chaininterface::BroadcasterInterface;

pub struct DBroadcaster();

impl DBroadcaster {
    pub fn new() -> DBroadcaster {
        DBroadcaster()
    }
}

impl BroadcasterInterface for DBroadcaster {
    fn broadcast_transaction(&self, tx: &Transaction) {
        // insert code to broadcast transaction
    }
}
