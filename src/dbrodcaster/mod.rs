use bitcoin::blockdata::transaction::Transaction;
use lightning::chain::chaininterface::BroadcasterInterface;

struct DBroadcaster();

impl BroadcasterInterface for DBroadcaster {
    fn broadcast_transaction(&self, tx: &Transaction) {
        // insert code to broadcast transaction
    }
}