use std::collections::HashMap;

use tx_indexer_primitives::traits::HasNLockTime;
use tx_indexer_primitives::traits::abstract_types::EnumerateOutputValueInArbitraryOrder;

#[derive(Debug, PartialEq, Eq)]
// TODO: use this instead of bool
pub enum TxCoinjoinAnnotation {
    CoinJoin,
    NotCoinJoin,
}

/// This is a super naive implementation that should be replace with a more sophisticated one.
#[derive(Debug)]
pub struct NaiveCoinjoinDetection;

impl NaiveCoinjoinDetection {
    pub fn is_coinjoin(tx: &impl EnumerateOutputValueInArbitraryOrder) -> bool {
        // If there are >= 3 outputs of the same value, tag as coinjoin.
        // TODO: impl actual detection
        let mut counts = HashMap::new();
        for value in tx.output_values() {
            *counts.entry(value).or_insert(0) += 1;
        }

        counts.values().any(|&count| count >= 3)
    }
}

#[derive(Debug)]
pub struct JoinMarketDetection;

impl JoinMarketDetection {
    /// Block height locktime. JoinMarket enforces `locktime > 0` (P2EP requirement).
    pub fn without_fidelity_bond(tx: &impl HasNLockTime) -> bool {
        tx.locktime() >= 1 && tx.locktime() <= 499_999_999
    }

    /// Unix timestamp locktime, defined by Bitcoin protocol >= 500_000_000.
    pub fn with_fidelity_bond(tx: &impl HasNLockTime) -> bool {
        tx.locktime() >= 500_000_000
    }
}

#[cfg(test)]
mod tests {

    use tx_indexer_primitives::test_utils::{DummyTxData, DummyTxOutData};

    use super::*;

    #[test]
    fn test_is_coinjoin_tx() {
        let not_coinjoin = DummyTxData {
            outputs: vec![
                DummyTxOutData::new_with_amount(100, 0),
                DummyTxOutData::new_with_amount(200, 1),
                DummyTxOutData::new_with_amount(300, 2),
            ],
            spent_coins: vec![],
            n_locktime: 0,
        };
        assert!(!NaiveCoinjoinDetection::is_coinjoin(&not_coinjoin));

        let coinjoin = DummyTxData {
            outputs: vec![
                DummyTxOutData::new_with_amount(100, 0),
                DummyTxOutData::new_with_amount(100, 1),
                DummyTxOutData::new_with_amount(100, 2),
                DummyTxOutData::new_with_amount(200, 3),
                DummyTxOutData::new_with_amount(200, 4),
                DummyTxOutData::new_with_amount(200, 5),
                DummyTxOutData::new_with_amount(300, 6),
                DummyTxOutData::new_with_amount(300, 7),
                DummyTxOutData::new_with_amount(300, 8),
            ],
            spent_coins: vec![],
            n_locktime: 0,
        };
        assert!(NaiveCoinjoinDetection::is_coinjoin(&coinjoin));
    }

    #[test]
    fn test_without_fidelity_bond() {
        let tx = DummyTxData::new(
            vec![DummyTxOutData::new_with_amount(100, 0)],
            vec![],
            800_000, // block height.
        );
        assert!(JoinMarketDetection::without_fidelity_bond(&tx));
        assert!(!JoinMarketDetection::with_fidelity_bond(&tx));
    }

    #[test]
    fn test_with_fidelity_bond() {
        let tx = DummyTxData::new(
            vec![DummyTxOutData::new_with_amount(100, 0)],
            vec![],
            1_700_000_000, // unix timestamp.
        );
        assert!(JoinMarketDetection::with_fidelity_bond(&tx));
        assert!(!JoinMarketDetection::without_fidelity_bond(&tx));
    }
    #[test]
    fn test_locktime_boundary() {
        let tx_max_height = DummyTxData::new(
            vec![DummyTxOutData::new_with_amount(100, 0)],
            vec![],
            499_999_999, // last valid block height.
        );
        assert!(JoinMarketDetection::without_fidelity_bond(&tx_max_height));
        assert!(!JoinMarketDetection::with_fidelity_bond(&tx_max_height));

        let tx_min_timestamp = DummyTxData::new(
            vec![DummyTxOutData::new_with_amount(100, 0)],
            vec![],
            500_000_000, // first valid unix timestamp.
        );
        assert!(JoinMarketDetection::with_fidelity_bond(&tx_min_timestamp));
        assert!(!JoinMarketDetection::without_fidelity_bond(
            &tx_min_timestamp
        ));
    }
    #[test]
    fn test_locktime_zero() {
        let tx = DummyTxData::new(vec![DummyTxOutData::new_with_amount(100, 0)], vec![], 0);
        assert!(!JoinMarketDetection::without_fidelity_bond(&tx));
        assert!(!JoinMarketDetection::with_fidelity_bond(&tx));
    }
}
