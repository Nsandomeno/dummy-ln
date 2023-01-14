use lightning::chain::chaininterface::{FeeEstimator, ConfirmationTarget};

pub struct DFeeEstimator();

impl FeeEstimator for DFeeEstimator {
    fn get_est_sat_per_1000_weight(&self, confirmation_target: ConfirmationTarget) -> u32 {
        match confirmation_target {
            ConfirmationTarget::Background => {9} // fetch background feerate,
            ConfirmationTarget::Normal => {8}  // fetch normal feerate (6 blocks)
            ConfirmationTarget::HighPriority => {7}  // fetch high priority feerate
        }
    }
}
